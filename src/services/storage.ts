import { invoke } from '@tauri-apps/api/core';

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
    // No explicit initialization needed for Celerix Store as it's lazy-loaded in Rust
    console.log('Storage: Celerix Store initialized');
  },

  async save<T>(key: keyof typeof STORAGE_KEYS, data: T): Promise<void> {
    try {
      console.log(`Saving ${key} to Celerix Store...`);
      await invoke('store_set', { key, value: data });
    } catch (e) {
      console.error(`Failed to save ${key}`, e);
      throw e;
    }
  },

  async load<T>(key: keyof typeof STORAGE_KEYS): Promise<T | null> {
    try {
      console.log(`Loading ${key} from Celerix Store...`);
      const data = await invoke<T | null>('store_get', { key });
      
      if (data !== null) {
        return data;
      }

      console.log(`${key} not found in Celerix Store, checking legacy storage...`);
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
        }
      }

      return null;
    } catch (e) {
      console.error(`Failed to load ${key}:`, e);
      throw e;
    }
  }
};
