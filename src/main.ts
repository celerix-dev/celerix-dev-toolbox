import 'halfmoon/css/halfmoon.min.css';
import 'halfmoon/css/cores/halfmoon.modern.css';
import 'bootstrap/dist/js/bootstrap.bundle.min.js';
import '@tabler/icons-webfont/dist/tabler-icons.min.css';
import { createSpectrum } from "celerix-spectrum/vue";
import './assets/main.css';

import '@/services/color-scheme.ts';
import { storageService } from '@/services/storage';

import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { relaunch } from '@tauri-apps/plugin-process';

import App from './App.vue';
import router from './router/index';

const app = createApp(App);

app.use(createSpectrum())

// Initialize storage before mounting
console.log('Main: Initializing storage...');
storageService.init().then(async () => {
  console.log('Main: Storage initialized, mounting app...');
  // Small settle delay for IPC/PTY cleanup during rapid reloads
  // This helps ensure the OS has time to release file handles or PTYs
  // before the new session tries to re-open them.
  const navigationEntry = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
  const isReload = navigationEntry && navigationEntry.type === 'reload';
  
  // In development mode (npm run tauri dev), we might want to avoid relaunching on every reload 
  // because it's slow and disrupts the dev experience. However, we must clear resources.
  // If it's dev, we'll try a standard reload first, but if it's the "hard" way, we relaunch.
  const isDev = import.meta.env.DEV;
  
  if (isReload) {
    if (isDev) {
      console.log('Main: Dev reload detected, allowing standard reload to preserve HMR/Dev experience.');
      // Fall through to normal app mount, but ensure we at least waited a bit
      await new Promise(resolve => setTimeout(resolve, 300));
    } else {
      console.log('Main: Production reload detected, performing full application relaunch...');
      try {
        // Stop the browser from trying to load further resources before we kill the process
        if (typeof window !== 'undefined') {
          window.stop();
          window.location.href = 'data:text/html,<html><body style="background:#1a1a1a;"></body></html>';
        }
        
        setTimeout(async () => {
          try {
            await relaunch();
          } catch (err) {
            console.error('Delayed relaunch failed', err);
            window.location.replace('/');
          }
        }, 250);
        return; 
      } catch (e) {
        console.error('Main: Failed to relaunch, falling back to delayed mount', e);
        await new Promise(resolve => setTimeout(resolve, 1500));
      }
    }
  } else {
    // Standard fresh start: short delay to ensure PTY/IPC bridge is ready
    await new Promise(resolve => setTimeout(resolve, 500));
  }
  
  app.use(createPinia());
  app.use(router);
  app.mount('#app');
});