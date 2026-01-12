<script setup lang="ts">
import { ref } from 'vue';
import { useUserStore } from '@/stores/user';
import { storageService } from '@/services/storage';
import AlertModal from '@/components/Basic/AlertModal.vue';
import ConfirmationModal from '@/components/Basic/ConfirmationModal.vue';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';

const userStore = useUserStore();

// UI State
const showAlert = ref(false);
const alertTitle = ref('');
const alertMessage = ref('');
const alertVariant = ref<'primary' | 'danger' | 'warning' | 'info' | 'success'>('primary');

const showConfirmReset = ref(false);

const triggerAlert = (title: string, message: string, variant: 'primary' | 'danger' | 'warning' | 'info' | 'success' = 'primary') => {
  alertTitle.value = title;
  alertMessage.value = message;
  alertVariant.value = variant;
  showAlert.value = true;
};

const handleThemeChange = (theme: 'auto' | 'light' | 'dark') => {
  userStore.setTheme(theme);
};

const exportAllData = async () => {
  try {
    const dataKeys = ['PROJECTS', 'KANBAN', 'TEMPLATES', 'LOGS', 'WIDGETS', 'USER'];
    const allData: Record<string, any> = {};
    
    for (const key of dataKeys) {
      allData[key] = await storageService.load(key as any);
    }
    
    const filePath = await save({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      defaultPath: `celerix-backup-${new Date().toISOString().split('T')[0]}.json`
    });

    if (filePath) {
      await writeTextFile(filePath, JSON.stringify(allData, null, 2));
      triggerAlert('Export Successful', 'All application data has been exported.', 'success');
    }
  } catch (e: any) {
    console.error('Export failed', e);
    triggerAlert('Export Failed', `Error: ${e.message}`, 'danger');
  }
};

const importAllData = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });

    if (selected) {
      const content = await readTextFile(selected as string);
      const imported = JSON.parse(content);
      
      // Basic validation
      if (typeof imported !== 'object' || Array.isArray(imported)) {
        throw new Error('Invalid backup file format.');
      }
      
      for (const [key, data] of Object.entries(imported)) {
        if (['PROJECTS', 'KANBAN', 'TEMPLATES', 'LOGS', 'WIDGETS', 'USER'].includes(key)) {
          await storageService.save(key as any, data);
        }
      }
      
      triggerAlert('Import Successful', 'Application data has been imported. Please restart or refresh the application to apply all changes.', 'success');
    }
  } catch (e: any) {
    console.error('Import failed', e);
    triggerAlert('Import Failed', `Error: ${e.message}`, 'danger');
  }
};

const resetApplication = async () => {
  showConfirmReset.value = true;
};

const handleConfirmReset = async () => {
  try {
    const dataKeys = ['PROJECTS', 'KANBAN', 'TEMPLATES', 'LOGS', 'WIDGETS', 'USER'];
    for (const key of dataKeys) {
      // In a real app we might want to delete the files, for now we save empty/null
      let defaultValue: any = null;
      if (key === 'USER') defaultValue = { nickname: 'Celerix Pilot', theme: 'auto', activeProjectId: null };
      if (key === 'PROJECTS') defaultValue = { version: '1.0.0', projects: [] };
      if (key === 'KANBAN') defaultValue = { version: '1.0.0', columns: [] };
      
      await storageService.save(key as any, defaultValue);
    }
    triggerAlert('Application Reset', 'All data has been cleared. The app will now relaunch.', 'warning');
    setTimeout(() => {
        window.location.reload();
    }, 2000);
  } catch (e: any) {
    console.error('Reset failed', e);
    triggerAlert('Reset Failed', `Error: ${e.message}`, 'danger');
  } finally {
    showConfirmReset.value = false;
  }
};

</script>

<template>
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center gap-2">
      <i class="ti ti-settings"></i>
      <strong>Settings</strong>
    </div>
  </teleport>

  <div class="container py-4">
    <div class="row justify-content-center">
      <div class="col-12 col-lg-8">
        
        <!-- Appearance Section -->
        <section class="mb-5">
          <h5 class="mb-3 d-flex align-items-center gap-2">
            <i class="ti ti-palette text-primary"></i> Appearance
          </h5>
          <div class="card border-0 shadow-sm">
            <div class="card-body">
              <div class="d-flex justify-content-between align-items-center mb-3">
                <div>
                  <h6 class="mb-0">Color Mode</h6>
                  <small class="text-muted">Choose how Celerix looks to you.</small>
                </div>
                <div class="btn-group shadow-none" role="group">
                  <input type="radio" class="btn-check" name="theme-mode" id="theme-auto" autocomplete="off" :checked="userStore.theme === 'auto'" @change="handleThemeChange('auto')">
                  <label class="btn btn-outline-secondary btn-sm px-3" for="theme-auto">
                    <i class="ti ti-device-desktop me-1"></i> Auto
                  </label>

                  <input type="radio" class="btn-check" name="theme-mode" id="theme-light" autocomplete="off" :checked="userStore.theme === 'light'" @change="handleThemeChange('light')">
                  <label class="btn btn-outline-secondary btn-sm px-3" for="theme-light">
                    <i class="ti ti-sun me-1"></i> Light
                  </label>

                  <input type="radio" class="btn-check" name="theme-mode" id="theme-dark" autocomplete="off" :checked="userStore.theme === 'dark'" @change="handleThemeChange('dark')">
                  <label class="btn btn-outline-secondary btn-sm px-3" for="theme-dark">
                    <i class="ti ti-moon me-1"></i> Dark
                  </label>
                </div>
              </div>
            </div>
          </div>
        </section>

        <!-- Data Management Section -->
        <section class="mb-5">
          <h5 class="mb-3 d-flex align-items-center gap-2">
            <i class="ti ti-database text-primary"></i> Data Management
          </h5>
          <div class="card border-0 shadow-sm mb-3">
            <div class="card-body">
              <div class="d-flex justify-content-between align-items-center mb-4">
                <div>
                  <h6 class="mb-0">Full Backup</h6>
                  <small class="text-muted">Export or import all your application data (Projects, Kanban, Widgets, etc.) as a single file.</small>
                </div>
                <div class="d-flex gap-2">
                  <button class="btn btn-secondary btn-sm" @click="importAllData">
                    <i class="ti ti-download me-1"></i> Import
                  </button>
                  <button class="btn btn-primary btn-sm" @click="exportAllData">
                    <i class="ti ti-upload me-1"></i> Export All
                  </button>
                </div>
              </div>
              <hr class="text-muted opacity-25">
              <div class="d-flex justify-content-between align-items-center mt-4">
                <div>
                  <h6 class="mb-0 text-danger">Reset Application</h6>
                  <small class="text-muted">Warning: This will permanently delete all your projects, tasks, and settings.</small>
                </div>
                <button class="btn btn-outline-danger btn-sm" @click="resetApplication">
                  <i class="ti ti-trash me-1"></i> Reset All Data
                </button>
              </div>
            </div>
          </div>
        </section>

        <!-- About Section -->
        <section>
          <h5 class="mb-3 d-flex align-items-center gap-2">
            <i class="ti ti-info-circle text-primary"></i> About
          </h5>
          <div class="card border-0 shadow-sm">
            <div class="card-body">
              <div class="d-flex align-items-center gap-3 mb-3">
                <img src="/celerix-logo.png" alt="Logo" width="48" height="48" class="rounded shadow-sm">
                <div>
                  <h6 class="mb-0">Celerix Dev Toolbox</h6>
                  <small class="text-muted">Version 0.1.6</small>
                </div>
              </div>
              <p class="text-muted small">
                Celerix is a local-first development toolbox designed to streamline your workflow with integrated Kanban, Terminal, and utility tools.
              </p>
              <div class="d-flex gap-3">
                <a href="https://github.com" target="_blank" class="text-decoration-none small d-flex align-items-center gap-1 text-primary">
                  <i class="ti ti-brand-github"></i> GitHub
                </a>
                <a href="#" class="text-decoration-none small d-flex align-items-center gap-1 text-primary">
                  <i class="ti ti-file-text"></i> Documentation
                </a>
              </div>
            </div>
          </div>
        </section>

      </div>
    </div>
  </div>

  <AlertModal
    :show="showAlert"
    :title="alertTitle"
    :message="alertMessage"
    :variant="alertVariant"
    @close="showAlert = false"
  />

  <ConfirmationModal
    :show="showConfirmReset"
    title="Reset Application"
    message="Are you sure you want to delete ALL data? This cannot be undone."
    confirm-text="Reset Everything"
    variant="danger"
    @close="showConfirmReset = false"
    @confirm="handleConfirmReset"
  />
</template>

<style scoped>
.btn-check:checked + .btn-outline-secondary {
  background-color: var(--bs-primary);
  border-color: var(--bs-primary);
  color: white;
}

section h5 {
  font-weight: 600;
  font-size: 1rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>
