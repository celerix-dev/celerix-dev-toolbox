<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { Terminal, type ITheme, type IDisposable } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { spawn } from 'tauri-pty';
import { relaunch } from '@tauri-apps/plugin-process';
import '@xterm/xterm/css/xterm.css';
import {getCurrentWindow} from "@tauri-apps/api/window";

// 1. Define Props with TS
interface Props {
  showHeader?: boolean;
}
const props = withDefaults(defineProps<Props>(), {
  showHeader: true
});

const emit = defineEmits(['exit']);

// 2. Strongly Typed Refs and Variables
const terminalElement = ref<HTMLDivElement | null>(null);

type PtyProcess = ReturnType<typeof spawn>
let term: Terminal | null = null;
let fitAddon: FitAddon | null = null;
let pty: PtyProcess | null = null;

// Listeners in xterm/tauri-pty usually return an IDisposable
let onDataListener: IDisposable | null = null;
let ptyListener: IDisposable | null = null;
let exitListener: IDisposable | null = null;
let isUnmounting = false;
let respawnTimer: ReturnType<typeof setTimeout> | null = null;
let writeTimeout: ReturnType<typeof setTimeout> | null = null;

// 3. Typed Color Palettes
const darkPalette: ITheme = {
  background: '#00000000',
  foreground: '#acacac',
  cursor: '#acacac',
  black: '#000000',
  red: '#ff5555',
  green: '#50fa7b',
  yellow: '#f1fa8c',
  blue: '#bd93f9',
  magenta: '#ff79c6',
  cyan: '#8be9fd',
  white: '#bbbbbb',
};

const lightPalette: ITheme = {
  background: '#ffffff00',
  foreground: '#232323',
  cursor: '#232323',
  black: '#000000',
  red: '#cc0000',
  green: '#008800',
  yellow: '#888800',
  blue: '#0000cc',
  magenta: '#880088',
  cyan: '#008888',
  white: '#e5e5e5',
  brightBlack: '#666666',
  brightRed: '#e60000',
  brightGreen: '#00aa00',
  brightYellow: '#aaaa00',
  brightBlue: '#0000ff',
  brightMagenta: '#aa00aa',
  brightCyan: '#0598bc',
  brightWhite: '#ffffff',
};

let isStarting = false;

const startShell = async (): Promise<void> => {
  if (!term || !fitAddon || isUnmounting || isStarting) return;
  isStarting = true;

  try {
    // Cleanup existing
    onDataListener?.dispose();
    onDataListener = null;
    ptyListener?.dispose();
    ptyListener = null;
    exitListener?.dispose();
    exitListener = null;
    if (respawnTimer) {
      clearTimeout(respawnTimer);
      respawnTimer = null;
    }
    
    if (pty) {
      console.log('Terminal: Killing previous PTY...');
      try {
        const ptyToKill = pty;
        pty = null; // Null it immediately
        
        // Ensure no more data is processed from this PTY
        // Note: These were already nulled above, but we repeat for clarity
        // if they were somehow re-assigned (though they shouldn't be here)
        (onDataListener as IDisposable | null)?.dispose();
        onDataListener = null;
        (ptyListener as IDisposable | null)?.dispose();
        ptyListener = null;
        (exitListener as IDisposable | null)?.dispose();
        exitListener = null;

        ptyToKill.kill();
      } catch (e) {
        console.warn('Terminal: Error killing PTY during startShell', e);
      }
      // Essential delay to let the backend/OS clean up the PTY resource
      await new Promise(resolve => setTimeout(resolve, 200));
    }

    if (isUnmounting) return;

    const isWindows = window.navigator.userAgent.includes('Windows');
    const shell = isWindows ? 'powershell.exe' : '/bin/zsh';

    // Ensure we have at least SOME dimensions
    const cols = term.cols || 80;
    const rows = term.rows || 24;

    // Spawn the process
    console.log('Terminal: Spawning shell...', shell, { cols, rows });
    try {
      pty = spawn(shell, [], {
        cols,
        rows,
        env: {
          "TERM": 'xterm-256color',
          "COLORTERM": "truecolor",
        }
      });
      console.log('Terminal: Shell spawned successfully.');
    } catch (err) {
      console.error('Terminal: Failed to spawn shell', err);
      term.writeln('\x1b[31mFailed to spawn shell. Please check your system configuration.\x1b[0m');
      return;
    }

    // Pipe Data: System -> Screen
    let pendingDataChunks: Uint8Array[] = [];

    ptyListener = pty.onData((data: Uint8Array) => {
      if (isUnmounting || !term || !ptyListener) return;
      
      pendingDataChunks.push(data);

      if (!writeTimeout) {
        writeTimeout = setTimeout(() => {
          if (isUnmounting || !term) {
            pendingDataChunks = [];
            writeTimeout = null;
            return;
          }
          if (pendingDataChunks.length > 0) {
            // Calculate total length
            const totalLength = pendingDataChunks.reduce((acc, chunk) => acc + chunk.length, 0);
            
            // Optimization: if only one chunk, avoid copying
            if (pendingDataChunks.length === 1) {
              term.write(pendingDataChunks[0]);
            } else {
              const combinedData = new Uint8Array(totalLength);
              let offset = 0;
              for (const chunk of pendingDataChunks) {
                combinedData.set(chunk, offset);
                offset += chunk.length;
              }
              term.write(combinedData);
            }
            pendingDataChunks = [];
          }
          writeTimeout = null;
        }, 16); // Increased to 16ms (roughly 60fps) to better batch high-frequency output
      }
    });

    // Pipe Data: Keyboard -> System
    onDataListener = term.onData((data: string) => {
      pty?.write(data);
    });

    exitListener = pty.onExit(({ exitCode }) => {
      if (isUnmounting || !exitListener) {
        console.log('Terminal: Shell exited during unmount or listener already disposed, ignoring.');
        return;
      }
      console.log(`Terminal: Shell exited with code: ${exitCode}`);

      const currentWin = getCurrentWindow();
      const isDetached = currentWin.label.startsWith('term-');

      if (isDetached) {
        // It's a pop-out window: Emit exit event
        emit('exit');
      } else {
        term?.reset();
        term?.writeln('\x1b[33mProcess exited. Re-spawning...\x1b[0m');

        respawnTimer = setTimeout(() => {
          if (isUnmounting) return;
          term?.reset();
          startShell();
        }, 1000);
      }
    });

    // Final sync of dimensions with a slight delay to ensure read loop is ready
    await new Promise(resolve => setTimeout(resolve, 200));
    if (pty && term && !isUnmounting) {
      console.log('Terminal: Performing initial resize and prompt force...');
      try {
        fitAddon.fit();
        pty.resize(term.cols, term.rows);
        // Removed forced carriage return that caused double prompts on startup
      } catch (e) {
        console.warn('Terminal: Error during initial resize', e);
      }
    }
  } finally {
    isStarting = false;
  }
};

// Theme Management
let mediaQuery: MediaQueryList | null = null;
const handleThemeChange = (e: MediaQueryListEvent | MediaQueryList) => {
  if (term) {
    term.options.theme = e.matches ? darkPalette : lightPalette;
  }
};

onMounted(async () => {
  if (!terminalElement.value) return;

  // Reset flags
  isUnmounting = false;
  isStarting = false;

  window.addEventListener('beforeunload', handleBeforeUnload);
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

  term = new Terminal({
    cursorBlink: true,
    allowTransparency: true,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    fontSize: 13,
    lineHeight: 1.4,
    theme: mediaQuery.matches ? darkPalette : lightPalette,
  });

  mediaQuery.addEventListener('change', handleThemeChange);

  fitAddon = new FitAddon();
  term.loadAddon(fitAddon);
  term.open(terminalElement.value);

  // Ensure DOM is ready before fitting
  await nextTick();
  fitAddon.fit();
  term.focus();

  console.log('Terminal: Initialized, starting shell...');
  await startShell();

  window.addEventListener('resize', handleResize);
});

const handleResize = () => {
  fit();
}

const focus = () => {
  term?.focus();
}

const fit = () => {
  fitAddon?.fit();
  if (pty && term) pty.resize(term.cols, term.rows);
}

const reconnect = async () => {
  console.log('Terminal: Manual reconnect requested.');
  if (term) {
    term.reset();
    term.writeln('\x1b[33mRe-connecting...\x1b[0m');
  }
  await startShell();
}

const hardReset = async () => {
  console.log('Terminal: Hard reset requested via app relaunch.');
  cleanup(true);
  
  // In development mode, we still allow hard reset because the user might want 
  // to explicitly clear the 5-reload leak without closing the terminal.
  try {
    // Show a quick visual feedback before relaunching
    if (typeof window !== 'undefined') {
      window.stop();
      // Redirecting to a data URI is the most aggressive way to detach the WebView 
      // from the Tauri custom protocol server before it is terminated.
      window.location.href = 'data:text/html,<html><body style="background:#1a1a1a;color:#eee;display:flex;align-items:center;justify-content:center;height:100vh;font-family:sans-serif;">Restarting Celerix...</body></html>';
    }
    
    // Give the UI a moment to update and location change to take effect
    setTimeout(async () => {
      try {
        await relaunch();
      } catch (e) {
        console.error('Terminal: Relaunch failed during hard reset', e);
        window.location.replace('/');
      }
    }, 250);
  } catch (e) {
    console.error('Terminal: Error preparing hard reset', e);
    window.location.reload();
  }
}

defineExpose({
  focus,
  fit,
  reconnect,
  hardReset
});

const cleanup = (isSync = false) => {
  if (isUnmounting) return;
  isUnmounting = true;
  console.log('Terminal: Cleaning up resources (sync:', isSync, ')...');

  // 1. DISPOSE LISTENERS IMMEDIATELY (Synchronous)
  // This is the most important part to stop IPC flow before page unloads
  onDataListener?.dispose();
  onDataListener = null;
  ptyListener?.dispose();
  ptyListener = null;
  exitListener?.dispose();
  exitListener = null;

  if (respawnTimer) {
    clearTimeout(respawnTimer);
    respawnTimer = null;
  }
  if (writeTimeout) {
    clearTimeout(writeTimeout);
    writeTimeout = null;
  }
  window.removeEventListener('resize', handleResize);
  
  // 2. Kill the PTY process (Fire-and-forget if sync)
  try {
    if (pty) {
      const ptyToKill = pty;
      pty = null;
      console.log('Terminal: Requesting PTY kill for PID:', ptyToKill.pid);
      // We don't await this in sync mode (unload)
      ptyToKill.kill();
    }
  } catch (e) {
    console.error('Terminal: Error killing PTY', e);
  }

  // 3. Dispose the terminal (Synchronous)
  if (term) {
    term.dispose();
    term = null;
  }

  if (mediaQuery) {
    mediaQuery.removeEventListener('change', handleThemeChange);
    mediaQuery = null;
  }
  console.log('Terminal: Cleanup logic finished.');
};

const handleBeforeUnload = () => {
  // During beforeunload, we must be strictly synchronous
  cleanup(true);
};

onBeforeUnmount(async () => {
  window.removeEventListener('beforeunload', handleBeforeUnload);
  cleanup();
});
</script>

<template>
  <div class="terminal-wrapper">
    <div v-if="props.showHeader" class="terminal-title d-flex justify-content-between align-items-center">
      <div class="d-flex align-items-center gap-3">
        <span>Celerix Terminal</span>
        <button class="btn btn-xs btn-outline-secondary py-0 px-1 border-0 shadow-none no-focus-ring" @click="reconnect" title="Reconnect Terminal">
          <i class="ti ti-refresh"></i>
        </button>
        <button class="btn btn-xs btn-outline-danger py-0 px-1 border-0 shadow-none no-focus-ring" @click="hardReset" title="Hard Reset App (Relaunch)">
          <i class="ti ti-power"></i>
        </button>
      </div>
      <small class="text-secondary smaller">
        <i class="ti ti-info-circle"></i> Closing/Hiding terminates session
      </small>
    </div>
    <div ref="terminalElement" class="terminal-container"></div>
  </div>
</template>

<style scoped>
.terminal-wrapper {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.xterm-viewport),
:deep(.xterm-screen) {
  background-color: transparent !important;
}

.terminal-container {
  flex-grow: 1;
  border-top: 1px solid var(--bs-border-color);
  background: var(--bs-tertiary-bg);
  padding: 8px;
  overflow: hidden;
}

.terminal-title {
  border: 1px solid var(--bs-border-color);
  border-left: 0;
  border-bottom: 0;
  padding: 4px 10px;
  background-color: var(--bs-secondary-bg);
  font-size: 12px;
  color: var(--bs-body-color);
}
</style>