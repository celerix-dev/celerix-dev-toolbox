<script setup lang="ts">

import ClockWidget from "@/components/Widgets/ClockWidget.vue";
import CountdownWidget from "@/components/Widgets/CountdownWidget.vue";
import AddWidgetModal from "@/components/Widgets/AddWidgetModal.vue";
import {colorScheme} from "@/services/color-scheme.ts";
import {onMounted, onBeforeUnmount, ref, watch} from "vue";
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import draggable from "vuedraggable";
import { eventBus } from "@/services/events";

interface Widget {
  id: string;
  type: 'countdown' | 'clock';
  label: string;
  targetDate?: string;
  isConfigured: boolean;
  clockOptions?: {
    showDate: boolean;
    showDigital: boolean;
    show24h: boolean;
  };
  countdownItems?: {
    id: string;
    label: string;
    targetDate: string;
  }[];
}

const widgets = ref<Widget[]>([]);

const loadWidgets = () => {
  const saved = localStorage.getItem('celerix-dashboard-widgets');
  if (saved) {
    try {
      widgets.value = JSON.parse(saved);
    } catch (e) {
      console.error('Failed to load widgets', e);
    }
  }

  // Ensure clock widget exists if list is empty
  if (widgets.value.length === 0) {
    widgets.value.push({
      id: 'default-clock',
      type: 'clock',
      label: 'Welcome',
      isConfigured: true,
      clockOptions: {
        showDate: false,
        showDigital: true,
        show24h: false
      }
    });
  }
};

const saveWidgets = () => {
  localStorage.setItem('celerix-dashboard-widgets', JSON.stringify(widgets.value));
};

watch(widgets, saveWidgets, { deep: true });

onMounted(() => {
  colorScheme.updateTheme();
  loadWidgets();
  eventBus.on('reset-dashboard', handleReset);
});

onBeforeUnmount(() => {
  eventBus.off('reset-dashboard', handleReset);
});

const handleReset = () => {
  localStorage.removeItem('celerix-dashboard-widgets');
  widgets.value = [];
  loadWidgets();
};

const addWidget = (type: string) => {
  if (type === 'countdown') {
    widgets.value.push({
      id: crypto.randomUUID(),
      type: 'countdown',
      label: 'Countdown',
      isConfigured: false,
      countdownItems: []
    });
  } else if (type === 'clock') {
    widgets.value.push({
      id: crypto.randomUUID(),
      type: 'clock',
      label: 'Clock',
      isConfigured: false,
      clockOptions: {
        showDate: false,
        showDigital: true,
        show24h: false
      }
    });
  }
};

const updateWidget = (updatedWidget: Widget) => {
  const index = widgets.value.findIndex(w => w.id === updatedWidget.id);
  if (index !== -1) {
    widgets.value[index] = updatedWidget;
  }
};

const removeWidget = (id: string) => {
  widgets.value = widgets.value.filter(w => w.id !== id);
};

const exportWidgets = async () => {
  try {
    const filePath = await save({
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }],
      defaultPath: 'celerix-widgets.json'
    });

    if (filePath) {
      await writeTextFile(filePath, JSON.stringify(widgets.value, null, 2));
    }
  } catch (e) {
    console.error('Failed to export widgets', e);
  }
};

const importWidgets = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (selected) {
      const content = await readTextFile(selected);
      const imported = JSON.parse(content);
      
      // Basic validation
      if (Array.isArray(imported)) {
        // We could do more thorough validation here if needed
        widgets.value = imported;
      }
    }
  } catch (e) {
    console.error('Failed to import widgets', e);
  }
};

</script>

<template>

  <div class="dropdown">
    <button class="btn btn-primary dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
      Button
    </button>
    <ul class="dropdown-menu">
      <li><h6 class="dropdown-header">Header</h6></li>
      <li><a href="#" class="dropdown-item active" aria-current="true">Dropdown link 1</a></li>
      <li><a href="#" class="dropdown-item">Dropdown link 2</a></li>
      <li><a href="#" class="dropdown-item">Dropdown link 3</a></li>
      <li><hr class="dropdown-divider"></li>
      <li><a class="dropdown-item disabled">Dropdown link 4</a></li>
    </ul>
  </div>
  
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center justify-content-between w-100">
      <div><i class="ti ti-dashboard"></i> <strong>Dashboard</strong></div>
    </div>
  </teleport>

  <teleport to="#page-context">
    <div class="btn-group">
      <button class="btn btn-secondary" @click="importWidgets" title="Import Widgets">
        <i class="ti ti-download"></i> Import
      </button>
      <button class="btn btn-secondary" @click="exportWidgets" title="Export Widgets">
        <i class="ti ti-upload"></i> Export
      </button>
    </div>
  </teleport>

  <div class="p-2">
    <draggable 
      v-model="widgets" 
      item-key="id" 
      tag="div"
      class="row g-3"
      handle=".card-header"
      ghost-class="ghost-widget"
      animation="200"
      :force-fallback="true"
      draggable=".widget-col"
      :fallback-tolerance="3"
    >
      <template #item="{ element: widget }">
        <div class="col-12 col-md-6 col-lg-4 col-xl-3 widget-col">
          <ClockWidget 
            v-if="widget.type === 'clock'" 
            :widget="widget"
            @update="updateWidget"
            @remove="removeWidget"
          />
          <CountdownWidget 
            v-else-if="widget.type === 'countdown'" 
            :widget="widget" 
            @update="updateWidget"
            @remove="removeWidget"
          />
          <div v-else class="card border-danger h-100 widget-card">
            <div class="card-header bg-danger text-white d-flex justify-content-between align-items-center">
              <span><i class="ti ti-alert-triangle"></i> Unknown Widget</span>
              <button class="btn btn-sm btn-link text-white p-0" @click="removeWidget(widget.id)">
                <i class="ti ti-trash"></i>
              </button>
            </div>
            <div class="card-body d-flex flex-column justify-content-center align-items-center text-center">
              <p class="mb-0 text-danger">The widget type <strong>"{{ widget.type }}"</strong> is not recognized.</p>
              <small class="text-muted mt-2">ID: {{ widget.id }}</small>
            </div>
          </div>
        </div>
      </template>
      
      <template #footer>
        <!-- Add Widget Button -->
        <div class="col-12 col-md-6 col-lg-4 col-xl-3">
          <div class="card add-widget-card h-100" style="min-height: 200px; cursor: pointer" data-bs-toggle="modal" data-bs-target="#addWidgetModal">
            <div class="card-body d-flex flex-column justify-content-center align-items-center text-muted">
              <i class="ti ti-plus fs-1"></i>
              <span>Add Widget</span>
            </div>
          </div>
        </div>
      </template>
    </draggable>
  </div>

  <AddWidgetModal @add="addWidget" />
</template>

<style scoped>
.add-widget-card:hover {
  background-color: var(--bs-secondary-bg);
}

.ghost-widget {
  opacity: 0.5;
  background: var(--bs-secondary-bg);
  border: 2px dashed var(--bs-primary);
}

:deep(.card-header) {
  cursor: grab;
}

:deep(.card-header:active) {
  cursor: grabbing;
}
</style>