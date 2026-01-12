
import { writeTextFile, readTextFile, exists, mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';

const STORAGE_KEYS = {
  PROJECTS: 'projects.json',
  KANBAN: 'kanban.json',
  TEMPLATES: 'templates.json',
  LOGS: 'logs.json',
  WIDGETS: 'widgets.json',
  USER: 'user.json'
};

const LEGACY_KEYS = {
  PROJECTS: 'celerix-projects',
  KANBAN: 'celerix-kanban-data',
  TEMPLATES: 'celerix-kanban-templates',
  LOGS: 'celerix-kanban-logs',
  WIDGETS: 'celerix-dashboard-widgets',
  USER: 'celerix-user-persona'
};

export const storageService = {
  async init() {
    try {
      // Clear any potential orphaned PTYs by calling a hypothetical cleanup command 
      // or just ensure we have a clean slate. 
      // Since we are using tauri-pty, we can try to kill all if the plugin supports it,
      // but usually we just want to ensure our own storage is ready.
      console.log('Storage: Initializing data directory...');
      
      // In Tauri v2, we should use a dot or a sub-path to check for directory existence
      const dataDirExists = await exists('data', { baseDir: BaseDirectory.AppData });
      if (!dataDirExists) {
        await mkdir('data', { baseDir: BaseDirectory.AppData, recursive: true });
      }
    } catch (e) {
      console.error('Failed to initialize storage directory', e);
    }
  },

  async save<T>(key: keyof typeof STORAGE_KEYS, data: T): Promise<void> {
    try {
      const fileName = `data/${STORAGE_KEYS[key]}`;
      console.log(`Saving ${key} to ${fileName}...`);
      await writeTextFile(fileName, JSON.stringify(data, null, 2), { baseDir: BaseDirectory.AppData });
    } catch (e) {
      console.error(`Failed to save ${key}`, e);
      throw e;
    }
  },

  async load<T>(key: keyof typeof STORAGE_KEYS): Promise<T | null> {
    const start = performance.now();
    const fileName = `data/${STORAGE_KEYS[key]}`;
    try {
      console.log(`[${start.toFixed(2)}] Loading ${key} from ${fileName}...`);
      
      let fileExists = false;
      let checkError: any = null;
      
      // Try to check existence
      try {
        fileExists = await exists(fileName, { baseDir: BaseDirectory.AppData });
      } catch (e) {
        checkError = e;
        console.warn(`[${performance.now().toFixed(2)}] Error checking if ${fileName} exists:`, e);
      }
      
      if (!fileExists && checkError) {
        // If we got an error and it doesn't "exist", wait and retry once
        console.log(`[${performance.now().toFixed(2)}] Retrying existence check for ${key}...`);
        await new Promise(resolve => setTimeout(resolve, 200));
        try {
          fileExists = await exists(fileName, { baseDir: BaseDirectory.AppData });
        } catch (e: any) {
          console.error(`[${performance.now().toFixed(2)}] Final existence check failed for ${key}`, e);
          throw new Error(`FS_CHECK_FAILED: ${e.message || e}`);
        }
      }
      
      if (fileExists) {
        const content = await readTextFile(fileName, { baseDir: BaseDirectory.AppData });
        const end = performance.now();
        console.log(`[${end.toFixed(2)}] ${key} loaded successfully. Length: ${content.length} chars. Duration: ${(end - start).toFixed(2)}ms`);
        
        if (!content || content.trim() === '') {
          console.error(`[${performance.now().toFixed(2)}] File ${fileName} is empty!`);
          throw new Error('FILE_EMPTY');
        }
        
        try {
          return JSON.parse(content) as T;
        } catch (e: any) {
          console.error(`[${performance.now().toFixed(2)}] JSON Parse error for ${key}:`, e);
          throw new Error(`JSON_PARSE_ERROR: ${e.message}`);
        }
      }

      console.log(`[${performance.now().toFixed(2)}] ${key} file not found at ${fileName}, checking legacy storage...`);
      const legacyKey = LEGACY_KEYS[key];
      const legacyData = localStorage.getItem(legacyKey);
      if (legacyData) {
        try {
          const parsed = JSON.parse(legacyData);
          console.log(`Found legacy data for ${key}, migrating...`);
          await this.save(key, parsed);
          return parsed as T;
        } catch (e) {
          console.error(`Failed to parse legacy data for ${key}`, e);
          // Don't throw here, just fall through to null if legacy is broken too
        }
      }

      console.log(`[${performance.now().toFixed(2)}] No data found for ${key}.`);
      return null;
    } catch (e) {
      const end = performance.now();
      console.error(`[${end.toFixed(2)}] CRITICAL LOAD FAILURE for ${key} after ${(end - start).toFixed(2)}ms:`, e);
      throw e;
    }
  }
};
