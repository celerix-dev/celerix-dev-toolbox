<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { Terminal, type ITheme, type IDisposable } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { spawn } from 'tauri-pty';
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

const startShell = async (): Promise<void> => {
  if (!term || !fitAddon) return;

  // Cleanup existing
  onDataListener?.dispose();
  ptyListener?.dispose();
  if (pty) pty.kill();

  const isWindows = window.navigator.userAgent.includes('Windows');
  const shell = isWindows ? 'powershell.exe' : '/bin/zsh';

  // Spawn the process
  pty = spawn(shell, [], {
    cols: term.cols,
    rows: term.rows,
    env: {
      "TERM": 'xterm-256color',
      "COLORTERM": "truecolor",
    }
  });

  // Pipe Data: System -> Screen
  ptyListener = ptyListener = pty.onData((data: Uint8Array) => term?.write(data));

  // Pipe Data: Keyboard -> System
  onDataListener = term.onData((data: string) => pty?.write(data));

  pty.onExit(({ exitCode }) => {
    console.log(`Shell exited with code: ${exitCode}`);

    const currentWin = getCurrentWindow();
    const isDetached = currentWin.label.startsWith('term-');

    if (isDetached) {
      // It's a pop-out window: Emit exit event
      emit('exit');
    } else {
      term?.reset();
      term?.writeln('\x1b[33mProcess exited. Re-spawning...\x1b[0m');

      setTimeout(() => {
        term?.reset();
        startShell();
      }, 1000);
    }
  });
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

defineExpose({
  focus,
  fit
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize);
  pty?.kill();
  onDataListener?.dispose();
  ptyListener?.dispose();
  term?.dispose();

  if (mediaQuery) {
    mediaQuery.removeEventListener('change', handleThemeChange);
  }
});
</script>

<template>
  <div class="terminal-wrapper">
    <div v-if="props.showHeader" class="terminal-title">Celerix Terminal</div>
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