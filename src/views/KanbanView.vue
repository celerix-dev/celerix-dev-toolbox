<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import draggable from 'vuedraggable';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import KanbanColumnComp from '@/components/Kanban/KanbanColumn.vue';

interface KanbanCard {
  id: string;
  title: string;
  description: string;
  color?: string;
}

interface KanbanColumn {
  id: string;
  title: string;
  color?: string;
  cards: KanbanCard[];
}

const columns = ref<KanbanColumn[]>([]);

const loadKanban = () => {
  const saved = localStorage.getItem('celerix-kanban-data');
  if (saved) {
    try {
      columns.value = JSON.parse(saved);
    } catch (e) {
      console.error('Failed to load kanban', e);
    }
  }

  if (columns.value.length === 0) {
    columns.value = [
      { id: 'todo', title: 'Todo', color: 'primary', cards: [] },
      { id: 'in-progress', title: 'In Progress', color: 'warning', cards: [] },
      { id: 'ready', title: 'Ready', color: 'success', cards: [] }
    ];
  }
};

const saveKanban = () => {
  localStorage.setItem('celerix-kanban-data', JSON.stringify(columns.value));
};

watch(columns, saveKanban, { deep: true });

onMounted(() => {
  loadKanban();
});

const addColumn = () => {
  columns.value.push({
    id: crypto.randomUUID(),
    title: 'New Column',
    color: 'secondary',
    cards: []
  });
};

const removeColumn = (id: string) => {
  columns.value = columns.value.filter(c => c.id !== id);
};

const addCard = (columnId: string) => {
  const column = columns.value.find(c => c.id === columnId);
  if (column) {
    column.cards.push({
      id: crypto.randomUUID(),
      title: 'New Task',
      description: '',
      color: 'light'
    });
  }
};

const removeCard = (columnId: string, cardId: string) => {
  const column = columns.value.find(c => c.id === columnId);
  if (column) {
    column.cards = column.cards.filter(c => c.id !== cardId);
  }
};

const exportKanban = async () => {
  try {
    const filePath = await save({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      defaultPath: 'celerix-kanban.json'
    });
    if (filePath) {
      await writeTextFile(filePath, JSON.stringify(columns.value, null, 2));
    }
  } catch (e) {
    console.error('Failed to export kanban', e);
  }
};

const importKanban = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });
    if (selected) {
      const content = await readTextFile(selected as string);
      const imported = JSON.parse(content);
      if (Array.isArray(imported)) {
        columns.value = imported;
      }
    }
  } catch (e) {
    console.error('Failed to import kanban', e);
  }
};

const updateCard = (columnId: string, updatedCard: KanbanCard) => {
  const column = columns.value.find(c => c.id === columnId);
  if (column) {
    const cardIndex = column.cards.findIndex(c => c.id === updatedCard.id);
    if (cardIndex !== -1) {
      column.cards[cardIndex] = updatedCard;
    }
  }
};

const colorOptions = ['primary', 'secondary', 'success', 'danger', 'warning', 'info', 'light', 'dark'];

</script>

<template>
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center justify-content-between w-100">
      <div><i class="ti ti-layout-kanban"></i> <strong>Kanban Board</strong></div>
    </div>
  </teleport>

  <teleport to="#page-context">
    <div class="btn-group">
      <button class="btn btn-secondary" @click="importKanban" title="Import Kanban">
        <i class="ti ti-download"></i> Import
      </button>
      <button class="btn btn-secondary" @click="exportKanban" title="Export Kanban">
        <i class="ti ti-upload"></i> Export
      </button>
      <button class="btn btn-primary" @click="addColumn" title="Add Column">
        <i class="ti ti-plus"></i> Add Column
      </button>
    </div>
  </teleport>

  <div class="kanban-container p-2">
    <draggable 
      v-model="columns" 
      item-key="id" 
      tag="div"
      class="kanban-row d-flex gap-3"
      handle=".column-header"
      ghost-class="ghost-column"
      animation="200"
      :force-fallback="true"
      draggable=".kanban-column"
      :fallback-tolerance="3"
    >
      <template #item="{ element: column }">
        <KanbanColumnComp 
          :column="column" 
          :color-options="colorOptions" 
          @remove="removeColumn"
          @add-card="addCard"
          @remove-card="removeCard"
          @update-card="updateCard"
        />
      </template>
    </draggable>
  </div>
</template>

<style scoped>
.kanban-container {
  height: 100%;
  overflow-x: auto;
  overflow-y: hidden;
}
.kanban-row {
  height: calc(100% - 10px);
}
.ghost-column {
  opacity: 0.5;
  background: var(--bs-secondary-bg);
  border: 2px dashed var(--bs-primary);
}
</style>
