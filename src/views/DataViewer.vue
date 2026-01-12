<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { storageService } from '@/services/storage';
import CodeEditor from '@/components/Basic/CodeEditor.vue';

const dataSources = [
  { key: 'PROJECTS', label: 'Projects', file: 'projects.json', info: 'Projects categorize work across the app. Kanban cards reference projects by their ID.' },
  { key: 'KANBAN', label: 'Kanban Board', file: 'kanban.json', info: 'Main Kanban data including columns and cards. Cards may reference Project IDs.' },
  { key: 'TEMPLATES', label: 'Card Templates', file: 'templates.json', info: 'Saved card configurations for reuse.' },
  { key: 'LOGS', label: 'Activity Logs', file: 'logs.json', info: 'Audit trail for Kanban cards, indexed by Card ID.' },
  { key: 'WIDGETS', label: 'Dashboard Widgets', file: 'widgets.json', info: 'Configuration for clock and countdown widgets.' }
];

const selectedSource = ref(dataSources[0]);
const rawData = ref<any>(null);
const flattenedData = ref<any[]>([]);
const columns = ref<string[]>([]);
const isLoading = ref(false);
const viewerModalData = ref<any>(null);
const storageStatus = ref<'ok' | 'error'>('ok');

const loadData = async () => {
  isLoading.value = true;
  storageStatus.value = 'ok';
  try {
    const data = await storageService.load<any>(selectedSource.value.key as any);
    rawData.value = data;
    processData(data);
  } catch (e) {
    console.error('Failed to load data', e);
    storageStatus.value = 'error';
  } finally {
    isLoading.value = false;
  }
};

const processData = (data: any) => {
  if (!data) {
    flattenedData.value = [];
    columns.value = [];
    return;
  }

  let items: any[] = [];
  
  // Try to find the main array in versioned envelopes
  if (data.projects) items = data.projects;
  else if (data.columns) items = data.columns;
  else if (Array.isArray(data)) items = data;
  else if (typeof data === 'object') {
    // For LOGS or single objects, we might need different handling
    // For now, let's treat LOGS (Record<string, log[]>) as items where key is cardId
    if (selectedSource.value.key === 'LOGS') {
        items = Object.entries(data).map(([cardId, logs]) => ({ cardId, logsCount: (logs as any[]).length, logs }));
    } else {
        items = [data];
    }
  }

  flattenedData.value = items;
  
  // Extract columns from the first item
  if (items.length > 0) {
    columns.value = Object.keys(items[0]).filter(key => typeof items[0][key] !== 'object' || items[0][key] === null);
  } else {
    columns.value = [];
  }
};

onMounted(loadData);

watch(selectedSource, loadData);

const openViewerModal = (item: any) => {
  viewerModalData.value = item;
};

const viewerModalDataJSON = computed(() => {
  return viewerModalData.value ? JSON.stringify(viewerModalData.value, null, 2) : '';
});

</script>

<template>
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center gap-2">
      <i class="ti ti-database"></i>
      <strong>Data Viewer</strong>
    </div>
  </teleport>

  <div class="container-fluid py-4">
    <div class="row mb-4">
      <div class="col-md-4">
        <label class="form-label">Data Source</label>
        <select v-model="selectedSource" class="form-select shadow-none">
          <option v-for="source in dataSources" :key="source.key" :value="source">
            {{ source.label }} ({{ source.file }})
          </option>
        </select>
      </div>
      <div class="col-md-8 d-flex align-items-end justify-content-end gap-2 text-muted small">
        <span v-if="storageStatus === 'error'" class="text-danger me-3">
          <i class="ti ti-alert-triangle"></i> IPC/Storage Error Detected
        </span>
        <span><i class="ti ti-lock"></i> Read-only mode</span>
      </div>
    </div>

    <div class="row mb-4" v-if="storageStatus === 'error'">
        <div class="col-12">
            <div class="alert alert-danger shadow-sm border-0 d-flex align-items-center gap-3">
                <i class="ti ti-alert-octagon fs-3"></i>
                <div>
                    <h6 class="alert-heading mb-1">System Congestion Detected</h6>
                    <p class="mb-0 small">
                        The application is experiencing delays communicating with the backend (IPC Bridge). 
                        This usually happens after rapid reloads with an active terminal. 
                        <strong>Please restart the application if data fails to appear.</strong>
                    </p>
                </div>
            </div>
        </div>
    </div>

    <div class="row mb-4">
        <div class="col-12">
            <div class="card bg-light-subtle border-0 shadow-sm">
                <div class="card-body py-2 px-3">
                    <small class="text-muted">
                        <i class="ti ti-info-circle me-1"></i>
                        <strong>Helpful Info:</strong> {{ selectedSource.info }}
                    </small>
                </div>
            </div>
        </div>
    </div>

    <div class="card border-0 shadow-sm overflow-hidden">
      <div class="table-responsive" style="max-height: 70vh;">
        <table class="table table-hover align-middle mb-0">
          <thead class="table-light sticky-top">
            <tr>
              <th v-for="col in columns" :key="col">{{ col }}</th>
              <th v-if="flattenedData.length > 0">Complex Data</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="isLoading">
              <td :colspan="columns.length + 1" class="text-center py-5">
                <div class="spinner-border spinner-border-sm text-primary me-2"></div>
                Loading data...
              </td>
            </tr>
            <tr v-else-if="flattenedData.length === 0">
              <td :colspan="columns.length + 1" class="text-center py-5 text-muted">
                No data found in this source.
              </td>
            </tr>
            <tr v-for="(item, index) in flattenedData" :key="index">
              <td v-for="col in columns" :key="col" class="text-truncate" style="max-width: 200px;" :title="item[col]">
                {{ item[col] }}
              </td>
              <td>
                <button 
                  class="btn btn-sm btn-outline-secondary" 
                  @click="openViewerModal(item)"
                  data-bs-toggle="modal"
                  data-bs-target="#jsonViewerModal"
                >
                  <i class="ti ti-eye"></i> View JSON
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <!-- JSON Viewer Modal -->
  <div class="modal fade" id="jsonViewerModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-lg modal-dialog-centered">
      <div class="modal-content">
        <div class="modal-header py-2 px-3">
          <h6 class="modal-title">Item Details (JSON)</h6>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body p-0">
          <CodeEditor :content="viewerModalDataJSON" read-only />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.table-responsive {
    border-radius: 8px;
}
.smaller {
    font-size: 0.75rem;
}
</style>
