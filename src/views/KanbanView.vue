<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { Modal } from 'bootstrap';
import draggable from 'vuedraggable';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import KanbanColumnComp from '@/components/Kanban/KanbanColumn.vue';

interface KanbanCard {
  id: string;
  title: string;
  description: string;
  color?: string;
  projectId?: string;
  priority?: 'low' | 'medium' | 'high' | 'urgent';
  dueDate?: string;
  createdAt: number;
  assignee?: string;
}

interface KanbanColumn {
  id: string;
  title: string;
  color?: string;
  cards: KanbanCard[];
}

interface ProjectUser {
  id: string;
  firstName: string;
  lastName: string;
  nickname: string;
}

interface Project {
  id: string;
  name: string;
  tag: string;
  icon: string;
  color: string;
  users: ProjectUser[];
}

const columns = ref<KanbanColumn[]>([]);
const projects = ref<Project[]>([]);

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

const loadProjects = () => {
  const saved = localStorage.getItem('celerix-projects');
  if (saved) {
    try {
      projects.value = JSON.parse(saved);
    } catch (e) {
      console.error('Failed to load projects', e);
    }
  }
};

const saveKanban = () => {
  localStorage.setItem('celerix-kanban-data', JSON.stringify(columns.value));
};

watch(columns, saveKanban, { deep: true });

onMounted(() => {
  loadKanban();
  loadProjects();
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
      color: 'light',
      priority: 'medium',
      createdAt: Date.now()
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

const isEditingCard = ref(false);
const editingCard = ref<KanbanCard | null>(null);
const editingColumnId = ref<string | null>(null);

const openEditModal = (columnId: string, card: KanbanCard) => {
  editingColumnId.value = columnId;
  editingCard.value = { ...card };
  isEditingCard.value = true;
  
  const modal = new Modal(document.getElementById('cardEditModal') as HTMLElement);
  modal.show();
};

const saveCard = () => {
  if (editingCard.value && editingColumnId.value) {
    updateCard(editingColumnId.value, editingCard.value);
    const modalElement = document.getElementById('cardEditModal');
    const modal = Modal.getInstance(modalElement as HTMLElement);
    modal?.hide();
  }
};

const handleProjectChange = (projectId: string) => {
  if (!editingCard.value) return;
  
  const oldProjectId = editingCard.value.projectId;
  const newProjectId = projectId === 'undefined' ? undefined : projectId;

  if (oldProjectId !== newProjectId) {
    const newProject = projects.value.find(p => p.id === newProjectId);
    if (newProject) {
      const isAssigneeInNewProject = newProject.users.some(u => u.nickname === editingCard.value?.assignee);
      if (!isAssigneeInNewProject && editingCard.value.assignee) {
        editingCard.value.assignee = '';
      }
    }
    editingCard.value.projectId = newProjectId;
  }
};

const colorOptions = ['primary', 'secondary', 'success', 'danger', 'warning', 'info', 'light', 'dark'];

</script>

<template>
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center justify-content-between w-100">
      <div class="d-flex align-items-center gap-2">
        <i class="ti ti-layout-kanban"></i>
        <strong>Kanban Board</strong>
      </div>
    </div>
  </teleport>

  <teleport to="#page-context">
    <div class="btn-group">
      <button class="btn btn-secondary d-flex align-items-center gap-1" @click="importKanban" title="Import Kanban">
        <i class="ti ti-download"></i> Import
      </button>
      <button class="btn btn-secondary d-flex align-items-center gap-1" @click="exportKanban" title="Export Kanban">
        <i class="ti ti-upload"></i> Export
      </button>
      <button class="btn btn-primary d-flex align-items-center gap-1" @click="addColumn" title="Add Column">
        <i class="ti ti-plus"></i> Add Column
      </button>
    </div>
  </teleport>

  <div class="kanban-container p-2">
    <draggable 
      v-model="columns" 
      item-key="id"
      class="kanban-row d-flex gap-3 align-items-start"
      handle=".column-header"
      ghost-class="ghost-column"
      :animation="200"
      :force-fallback="true"
      :fallback-tolerance="3"
    >
      <template #item="{ element: column, index }">
        <KanbanColumnComp 
          :column="column" 
          :color-options="colorOptions" 
          :projects="projects"
          @remove="removeColumn"
          @add-card="addCard"
          @remove-card="removeCard"
          @update-card="updateCard"
          @edit-card="openEditModal"
          @update:column="(newCol) => columns[index] = newCol"
        />
      </template>
    </draggable>
  </div>

  <!-- Card Edit Modal -->
  <div class="modal fade" id="cardEditModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content" v-if="editingCard">
        <div class="modal-header">
          <h5 class="modal-title">Edit Card</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <div class="mb-3">
            <label class="form-label">Title</label>
            <input v-model="editingCard.title" class="form-control" />
          </div>
          <div class="mb-3">
            <label class="form-label">Description</label>
            <textarea v-model="editingCard.description" class="form-control" rows="3"></textarea>
          </div>
          
          <div class="row g-3 mb-3">
            <div class="col-6">
              <label class="form-label">Project</label>
              <select 
                :value="editingCard.projectId" 
                @change="handleProjectChange(($event.target as HTMLSelectElement).value)"
                class="form-select"
              >
                <option :value="undefined">No Project</option>
                <option v-for="project in projects" :key="project.id" :value="project.id">
                  {{ project.name }}
                </option>
              </select>
            </div>
            <div class="col-6">
              <label class="form-label">Priority</label>
              <select v-model="editingCard.priority" class="form-select">
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
                <option value="urgent">Urgent</option>
              </select>
            </div>
          </div>

          <div class="row g-3 mb-3">
            <div class="col-6">
              <label class="form-label">Due Date</label>
              <input type="date" v-model="editingCard.dueDate" class="form-control" />
            </div>
            <div class="col-6">
              <label class="form-label">Assignee</label>
              <input 
                type="text" 
                v-model="editingCard.assignee" 
                class="form-control" 
                list="modal-project-users-list"
                placeholder="Type or select..."
              />
              <datalist id="modal-project-users-list">
                <option v-for="user in projects.find(p => p.id === editingCard?.projectId)?.users" :key="user.id" :value="user.nickname">
                  {{ user.firstName }} {{ user.lastName }}
                </option>
              </datalist>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
          <button type="button" class="btn btn-primary" @click="saveCard">Save Changes</button>
        </div>
      </div>
    </div>
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
  background: #f8f9fa;
  border: 2px dashed #ccc;
}
</style>
