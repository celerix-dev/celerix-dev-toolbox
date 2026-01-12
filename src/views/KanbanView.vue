<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import draggable from 'vuedraggable';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import KanbanColumnComp from '@/components/Kanban/KanbanColumn.vue';
import ConfirmationModal from '@/components/Basic/ConfirmationModal.vue';
import AlertModal from '@/components/Basic/AlertModal.vue';
import { schemaService } from '@/services/schema';
import { storageService } from '@/services/storage';
import { useUserStore } from '@/stores/user';

const userStore = useUserStore();

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
  checklist?: { id: string; text: string; completed: boolean }[];
}

interface KanbanColumn {
  id: string;
  title: string;
  color?: string;
  purpose?: string;
  cards: KanbanCard[];
}

const columnPurposes = [
  { value: 'backlog', label: 'Backlog', icon: 'ti-list' },
  { value: 'todo', label: 'Todo', icon: 'ti-square-rounded' },
  { value: 'in-progress', label: 'In Progress', icon: 'ti-loader' },
  { value: 'review', label: 'Review', icon: 'ti-search' },
  { value: 'done', label: 'Done', icon: 'ti-square-rounded-check' },
  { value: 'custom', label: 'Custom', icon: 'ti-settings' }
];

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
const isInitialized = ref(false);
const projects = ref<Project[]>([]);
const templates = ref<KanbanCard[]>([]);
const cardLogs = ref<Record<string, { action: string, timestamp: number }[]>>({});
const searchQuery = ref('');

// Modals state
const showAlert = ref(false);
const alertTitle = ref('');
const alertMessage = ref('');
const alertVariant = ref<'primary' | 'danger' | 'warning' | 'info' | 'success'>('primary');

const showConfirmClear = ref(false);

const triggerAlert = (title: string, message: string, variant: 'primary' | 'danger' | 'warning' | 'info' | 'success' = 'primary') => {
  alertTitle.value = title;
  alertMessage.value = message;
  alertVariant.value = variant;
  showAlert.value = true;
};

const filteredColumns = computed(() => {
  if (!userStore.activeProjectId) return columns.value;

  return columns.value.map(col => ({
    ...col,
    cards: col.cards.filter(card => card.projectId === userStore.activeProjectId)
  }));
});

const addFilter = (token: string) => {
  if (searchQuery.value && !searchQuery.value.endsWith(' ')) {
    searchQuery.value += ' ' + token;
  } else {
    searchQuery.value += token;
  }
};

const loadKanban = async () => {
  try {
    const parsed = await storageService.load<any>('KANBAN');
    if (parsed) {
      const validation = schemaService.validateKanban(parsed);
      if (validation.valid) {
        columns.value = parsed.columns;
      } else {
        // Migration/Fallback
        if (Array.isArray(parsed)) {
          columns.value = parsed;
          console.log('Migrated kanban to versioned format');
        } else {
          console.error('Kanban data validation failed', validation.errors);
          if (parsed.columns) columns.value = parsed.columns;
        }
      }
    }

    const savedTemplates = await storageService.load<KanbanCard[]>('TEMPLATES');
    if (savedTemplates) {
      templates.value = savedTemplates;
    }

    const savedLogs = await storageService.load<Record<string, { action: string, timestamp: number }[]>>('LOGS');
    if (savedLogs) {
      cardLogs.value = savedLogs;
    }

    if (columns.value.length === 0) {
      columns.value = [
        { id: 'todo', title: 'Todo', color: 'primary', purpose: 'todo', cards: [] },
        { id: 'in-progress', title: 'In Progress', color: 'warning', purpose: 'in-progress', cards: [] },
        { id: 'ready', title: 'Ready', color: 'success', purpose: 'done', cards: [] }
      ];
    }
  } catch (e) {
    console.error('Error loading Kanban data', e);
    throw e;
  }
};

const loadProjects = async () => {
  try {
    const parsed = await storageService.load<any>('PROJECTS');
    if (parsed) {
      if (parsed && parsed.projects) {
        projects.value = parsed.projects;
      } else if (Array.isArray(parsed)) {
        projects.value = parsed;
      }
    }
  } catch (e) {
    console.error('Error loading projects for Kanban', e);
    throw e;
  }
};

const saveKanban = async () => {
  if (!isInitialized.value) return;
  const versioned = schemaService.getVersionedKanban(columns.value);
  await storageService.save('KANBAN', versioned);
};

const saveTemplates = async () => {
  if (!isInitialized.value) return;
  await storageService.save('TEMPLATES', templates.value);
};

const saveLogs = async () => {
  if (!isInitialized.value) return;
  await storageService.save('LOGS', cardLogs.value);
};

watch(columns, saveKanban, { deep: true });
watch(templates, saveTemplates, { deep: true });
watch(cardLogs, saveLogs, { deep: true });

onMounted(async () => {
  console.log('KanbanView: Starting initialization...');
  try {
    await Promise.all([
      loadKanban(),
      loadProjects()
    ]);
    isInitialized.value = true;
    console.log('KanbanView: Initialization successful.');
  } catch (e: any) {
    console.error('KanbanView: Failed to initialize', e);
    triggerAlert('Load Error', 'Failed to load Kanban data. Auto-save disabled to prevent data loss. Error: ' + e.message, 'danger');
  }
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

const addLog = (cardId: string, action: string) => {
  if (!cardLogs.value[cardId]) {
    cardLogs.value[cardId] = [];
  }
  cardLogs.value[cardId].unshift({
    action,
    timestamp: Date.now()
  });
  // Keep only last 50 logs per card
  if (cardLogs.value[cardId].length > 50) {
    cardLogs.value[cardId] = cardLogs.value[cardId].slice(0, 50);
  }
};

const addCard = (columnId: string) => {
  const column = columns.value.find(c => c.id === columnId);
  if (column) {
    const newCard = {
      id: crypto.randomUUID(),
      title: 'New Task',
      description: '',
      color: 'light',
      priority: 'medium' as const,
      createdAt: Date.now()
    };
    column.cards.push(newCard);
    addLog(newCard.id, 'Card created');
  }
};

const removeCard = (columnId: string, cardId: string) => {
  const column = columns.value.find(c => c.id === columnId);
  if (column) {
    column.cards = column.cards.filter(c => c.id !== cardId);
    // Cleanup logs
    delete cardLogs.value[cardId];
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
      const validation = schemaService.validateKanban(imported);
      if (validation.valid) {
        columns.value = imported.columns;
      } else {
        // Fallback for old format
        if (Array.isArray(imported)) {
          columns.value = imported;
        } else {
          triggerAlert('Import Failed', 'Invalid Kanban data structure.\n' + validation.errors.join('\n'), 'danger');
        }
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
      const oldCard = column.cards[cardIndex];
      // Detect changes for logging
      if (oldCard.title !== updatedCard.title) addLog(updatedCard.id, `Title changed to: ${updatedCard.title}`);
      if (oldCard.assignee !== updatedCard.assignee) addLog(updatedCard.id, `Assignee changed to: ${updatedCard.assignee || 'Unassigned'}`);
      if (oldCard.priority !== updatedCard.priority) addLog(updatedCard.id, `Priority changed to: ${updatedCard.priority}`);
      if (oldCard.projectId !== updatedCard.projectId) {
        const projectName = projects.value.find(p => p.id === updatedCard.projectId)?.name || 'No Project';
        addLog(updatedCard.id, `Project changed to: ${projectName}`);
      }
      
      column.cards[cardIndex] = updatedCard;
    }
  }
};

const isEditingCard = ref(false);
const editingCard = ref<KanbanCard | null>(null);
const editingColumnId = ref<string | null>(null);
const editingColumn = ref<KanbanColumn | null>(null);

const openEditModal = (columnId: string, card: KanbanCard) => {
  editingColumnId.value = columnId;
  editingCard.value = JSON.parse(JSON.stringify(card));
  if (!editingCard.value?.checklist) {
    editingCard.value!.checklist = [];
  }
  isEditingCard.value = true;
};

const addChecklistItem = () => {
  if (editingCard.value) {
    if (!editingCard.value.checklist) editingCard.value.checklist = [];
    editingCard.value.checklist.push({
      id: crypto.randomUUID(),
      text: '',
      completed: false
    });
  }
};

const removeChecklistItem = (id: string) => {
  if (editingCard.value?.checklist) {
    editingCard.value.checklist = editingCard.value.checklist.filter(item => item.id !== id);
  }
};

const openEditColumnModal = (column: KanbanColumn) => {
  editingColumn.value = JSON.parse(JSON.stringify(column));
};

const saveColumn = () => {
  if (editingColumn.value) {
    const index = columns.value.findIndex(c => c.id === editingColumn.value?.id);
    if (index !== -1) {
      columns.value[index] = { ...editingColumn.value };
    }
  }
};

const saveCard = () => {
  if (editingCard.value && editingColumnId.value) {
    updateCard(editingColumnId.value, editingCard.value);
  }
};

const saveAsTemplate = () => {
  if (editingCard.value) {
    const template = JSON.parse(JSON.stringify(editingCard.value));
    template.id = crypto.randomUUID();
    template.createdAt = Date.now();
    // Keep everything else as template data
    templates.value.push(template);
    triggerAlert('Success', 'Card saved as template!', 'success');
  }
};

const deleteTemplate = (id: string) => {
  templates.value = templates.value.filter(t => t.id !== id);
};

const useTemplate = (columnId: string, template: KanbanCard) => {
  const column = columns.value.find(c => c.id === columnId);
  if (column) {
    const newCard = JSON.parse(JSON.stringify(template));
    newCard.id = crypto.randomUUID();
    newCard.createdAt = Date.now();
    column.cards.push(newCard);
    addLog(newCard.id, `Card created from template: ${template.title}`);
  }
};

const clearCompleted = () => {
  showConfirmClear.value = true;
};

const handleClearDone = () => {
  columns.value = columns.value.map(col => {
    if (col.purpose === 'done') {
      return { ...col, cards: [] };
    }
    return col;
  });
  showConfirmClear.value = false;
};

const handleCardMoved = (cardId: string, _fromColumnId: string, toColumnId: string) => {
  const toColumn = columns.value.find(c => c.id === toColumnId);
  if (toColumn) {
    const purposeLabel = columnPurposes.find(p => p.value === toColumn.purpose)?.label || 'no specific purpose';
    addLog(cardId, `Moved to: ${toColumn.title} (${purposeLabel})`);
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
    <div class="d-flex align-items-center gap-2">
      <div class="input-group input-group-sm" style="width: 320px;">
        <span class="input-group-text bg-transparent border-end-0 text-muted">
          <i class="ti ti-search"></i>
        </span>
        <input 
          type="text" 
          v-model="searchQuery" 
          class="form-control border-start-0 border-end-0 shadow-none px-1" 
          placeholder="Filter cards..."
          @keyup.esc="searchQuery = ''"
        >
        <button class="btn btn-outline-secondary dropdown-toggle no-caret no-focus-ring" type="button" data-bs-toggle="dropdown" aria-expanded="false">
          <i class="ti ti-filter"></i>
        </button>
        <ul class="dropdown-menu dropdown-menu-end shadow-sm" style="width: 250px;">
          <li><h6 class="dropdown-header">Quick Filters</h6></li>
          <li><a class="dropdown-item" href="#" @click.prevent="addFilter('p:')" data-bs-dismiss="dropdown"><i class="ti ti-tag"></i> Project (p:name)</a></li>
          <li><a class="dropdown-item" href="#" @click.prevent="addFilter('pri:')" data-bs-dismiss="dropdown"><i class="ti ti-alert-circle"></i> Priority (pri:level)</a></li>
          <li><a class="dropdown-item" href="#" @click.prevent="addFilter('u:')" data-bs-dismiss="dropdown"><i class="ti ti-user"></i> User (u:nick)</a></li>
          <li v-if="projects.length > 0"><hr class="dropdown-divider"></li>
          <li v-if="projects.length > 0"><h6 class="dropdown-header">Projects</h6></li>
          <li v-for="p in projects" :key="p.id">
            <a class="dropdown-item d-flex align-items-center gap-2" href="#" @click.prevent="addFilter(`p:${p.tag}`)" data-bs-dismiss="dropdown">
              <span class="badge rounded-circle p-1" :style="{ backgroundColor: p.color }"></span>
              {{ p.name }} (#{{ p.tag }})
            </a>
          </li>
          <li><hr class="dropdown-divider"></li>
          <li><h6 class="dropdown-header">Priorities</h6></li>
          <li><a class="dropdown-item text-success" href="#" @click.prevent="addFilter('pri:low')" data-bs-dismiss="dropdown">Low</a></li>
          <li><a class="dropdown-item text-info" href="#" @click.prevent="addFilter('pri:medium')" data-bs-dismiss="dropdown">Medium</a></li>
          <li><a class="dropdown-item text-warning" href="#" @click.prevent="addFilter('pri:high')" data-bs-dismiss="dropdown">High</a></li>
          <li><a class="dropdown-item text-danger" href="#" @click.prevent="addFilter('pri:urgent')" data-bs-dismiss="dropdown">Urgent</a></li>
          <li><hr class="dropdown-divider"></li>
          <li><a class="dropdown-item text-danger" href="#" @click.prevent="searchQuery = ''" data-bs-dismiss="dropdown"><i class="ti ti-x"></i> Clear Filter</a></li>
        </ul>
      </div>
      <div class="btn-group">
        <button class="btn btn-secondary d-flex align-items-center gap-1" @click="clearCompleted" title="Clear Done Columns">
          <i class="ti ti-trash-x"></i> Clear Done
        </button>
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
    </div>
  </teleport>

  <div class="kanban-container p-2">
    <div v-if="userStore.activeProjectId" class="alert alert-info py-2 px-3 mb-2 d-flex align-items-center justify-content-between">
      <div class="d-flex align-items-center gap-2">
        <i class="ti ti-filter fs-5"></i>
        <span>Filtering by Project: <strong>{{ projects.find(p => p.id === userStore.activeProjectId)?.name }}</strong></span>
      </div>
      <button class="btn btn-sm btn-outline-info border-0 py-0" @click="userStore.setActiveProject(null)">Clear Filter</button>
    </div>
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
      <template #item="{ index }">
        <KanbanColumnComp 
          :column="filteredColumns[index]" 
          :color-options="colorOptions" 
          :projects="projects"
          :search-query="searchQuery"
          :templates="templates"
          @remove="removeColumn"
          @add-card="addCard"
          @remove-card="removeCard"
          @update-card="updateCard"
          @edit-card="openEditModal"
          @edit-column="openEditColumnModal"
          @update:column="(newCol) => columns[index] = newCol"
          @use-template="useTemplate"
          @delete-template="deleteTemplate"
          @card-moved="handleCardMoved"
        />
      </template>
    </draggable>
  </div>

  <ConfirmationModal
    :show="showConfirmClear"
    title="Clear Done Columns"
    message="Are you sure you want to clear all cards from 'Done' columns?"
    confirm-text="Clear Done"
    variant="danger"
    @close="showConfirmClear = false"
    @confirm="handleClearDone"
  />

  <AlertModal
    :show="showAlert"
    :title="alertTitle"
    :message="alertMessage"
    :variant="alertVariant"
    @close="showAlert = false"
  />

  <!-- Column Edit Modal -->
  <div class="modal fade" id="columnEditModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content" v-if="editingColumn">
        <div class="modal-header">
          <h5 class="modal-title">Edit Column</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <div class="mb-3">
            <label class="form-label">Column Title</label>
            <input v-model="editingColumn.title" class="form-control" placeholder="e.g. Done" />
          </div>
          <div class="mb-3">
            <label class="form-label">Column Purpose</label>
            <select v-model="editingColumn.purpose" class="form-select">
              <option :value="undefined">No Purpose</option>
              <option v-for="purpose in columnPurposes" :key="purpose.value" :value="purpose.value">
                {{ purpose.label }}
              </option>
            </select>
            <div class="form-text small" v-if="editingColumn.purpose">
              <i :class="['ti', columnPurposes.find(p => p.value === editingColumn?.purpose)?.icon]"></i>
              This column acts as <strong>{{ columnPurposes.find(p => p.value === editingColumn?.purpose)?.label }}</strong>.
            </div>
          </div>
          <div class="mb-3">
            <label class="form-label d-block">Header Color</label>
            <div class="d-flex flex-wrap gap-2">
              <div 
                v-for="color in colorOptions" 
                :key="color"
                :class="['color-box', `bg-${color}`, editingColumn.color === color ? 'active' : '']"
                @click="editingColumn.color = color"
              ></div>
            </div>
          </div>
        </div>
        <div class="modal-footer justify-content-between">
          <button type="button" class="btn btn-outline-danger" @click="removeColumn(editingColumn.id)" data-bs-dismiss="modal">
            <i class="ti ti-trash"></i> Delete Column
          </button>
          <div class="d-flex gap-2">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
            <button type="button" class="btn btn-primary" @click="saveColumn" data-bs-dismiss="modal">Save Changes</button>
          </div>
        </div>
      </div>
    </div>
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

          <div class="mb-3">
            <div class="d-flex justify-content-between align-items-center mb-2">
              <label class="form-label mb-0">Checklist</label>
              <button type="button" class="btn btn-sm btn-outline-primary" @click="addChecklistItem">
                <i class="ti ti-plus"></i> Add Item
              </button>
            </div>
            <div v-if="!editingCard.checklist || editingCard.checklist.length === 0" class="text-center py-2 bg-light rounded border border-dashed">
              <small class="text-muted">No checklist items.</small>
            </div>
            <div v-else class="checklist-items">
              <div v-for="item in editingCard.checklist" :key="item.id" class="d-flex align-items-center gap-2 mb-2">
                <input type="checkbox" v-model="item.completed" class="form-check-input mt-0">
                <input type="text" v-model="item.text" class="form-control form-control-sm" placeholder="Item text...">
                <button type="button" class="btn btn-sm btn-link text-danger p-0" @click="removeChecklistItem(item.id)">
                  <i class="ti ti-x"></i>
                </button>
              </div>
            </div>
          </div>

          <div class="mb-3">
            <label class="form-label d-block mb-2">Activity Log</label>
            <div class="activity-log p-2 bg-light rounded border">
              <div v-if="!cardLogs[editingCard.id] || cardLogs[editingCard.id].length === 0" class="text-center py-2">
                <small class="text-muted">No activity yet.</small>
              </div>
              <div v-else class="log-items">
                <div v-for="(log, idx) in cardLogs[editingCard.id]" :key="idx" class="log-item mb-1 pb-1 border-bottom">
                  <div class="d-flex justify-content-between align-items-center mb-1">
                    <small class="fw-bold">{{ log.action }}</small>
                    <small class="text-muted smaller">{{ new Date(log.timestamp).toLocaleString() }}</small>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer justify-content-between">
          <button type="button" class="btn btn-outline-info" @click="saveAsTemplate">
            <i class="ti ti-template"></i> Save as Template
          </button>
          <div class="d-flex gap-2">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
            <button type="button" class="btn btn-primary" @click="saveCard" data-bs-dismiss="modal">Save Changes</button>
          </div>
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
.color-box {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid rgba(0,0,0,0.1);
  transition: transform 0.1s;
}
.color-box:hover {
  transform: scale(1.1);
}
.color-box.active {
  border: 2px solid #fff;
  box-shadow: 0 0 0 1px #000;
}
.no-focus-ring:focus {
  outline: none !important;
  box-shadow: none !important;
}
.input-group:focus-within {
  box-shadow: none;
}
.input-group .btn-outline-secondary {
  border-color: var(--bs-border-color);
}
.input-group:focus-within .input-group-text,
.input-group:focus-within .form-control {
  border-color: #86b7fe;
}
.input-group:focus-within .btn-outline-secondary {
  border-color: #86b7fe;
}
.border-dashed {
  border-style: dashed !important;
}
.checklist-items {
  max-height: 200px;
  overflow-y: auto;
  padding-right: 5px;
}
.activity-log {
  max-height: 150px;
  overflow-y: auto;
  font-size: 0.85rem;
}
.log-item:last-child {
  border-bottom: none !important;
}
</style>
