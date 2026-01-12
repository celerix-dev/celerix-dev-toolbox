<script setup lang="ts">
import { computed } from 'vue';
import draggable from 'vuedraggable';
import KanbanCardComp from './KanbanCard.vue';

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

const props = defineProps<{
  column: KanbanColumn;
  colorOptions: string[];
  projects: Project[];
  searchQuery?: string;
  templates?: KanbanCard[];
}>();

const emit = defineEmits<{
  (e: 'remove', id: string): void;
  (e: 'addCard', columnId: string): void;
  (e: 'removeCard', columnId: string, cardId: string): void;
  (e: 'updateCard', columnId: string, card: KanbanCard): void;
  (e: 'editCard', columnId: string, card: KanbanCard): void;
  (e: 'editColumn', column: KanbanColumn): void;
  (e: 'update:column', column: KanbanColumn): void;
  (e: 'useTemplate', columnId: string, template: KanbanCard): void;
  (e: 'deleteTemplate', id: string): void;
  (e: 'cardMoved', cardId: string, fromColumnId: string, toColumnId: string): void;
}>();

const onDragChange = (evt: any) => {
  if (evt.added) {
    emit('cardMoved', evt.added.element.id, 'unknown', props.column.id);
  }
};

const cards = computed({
  get: () => props.column.cards,
  set: (newCards) => {
    emit('update:column', { ...props.column, cards: newCards });
  }
});

const filteredCards = computed(() => {
  if (!props.searchQuery) return cards.value;
  
  const query = props.searchQuery.toLowerCase();
  const parts = query.split(' ').filter(p => p.trim() !== '');
  
  return cards.value.filter(card => {
    const project = props.projects.find(p => p.id === card.projectId);
    
    // Check if ALL parts match something in the card
    return parts.every(part => {
      if (part.startsWith('p:')) {
        const pTag = part.slice(2);
        if (!pTag) return true; // Just "p:" typed, ignore until more
        return project?.tag.toLowerCase().includes(pTag) || project?.name.toLowerCase().includes(pTag);
      }
      
      if (part.startsWith('pri:')) {
        const pri = part.slice(4);
        if (!pri) return true;
        return card.priority?.toLowerCase().includes(pri);
      }
      
      if (part.startsWith('u:')) {
        const u = part.slice(2);
        if (!u) return true;
        return card.assignee?.toLowerCase().includes(u);
      }
      
      // Default search in title, description
      return (
        card.title.toLowerCase().includes(part) ||
        card.description.toLowerCase().includes(part) ||
        card.assignee?.toLowerCase().includes(part) ||
        project?.name.toLowerCase().includes(part) ||
        project?.tag.toLowerCase().includes(part)
      );
    });
  });
});

const handleUpdateCard = (card: KanbanCard) => {
  emit('updateCard', props.column.id, card);
};
</script>

<template>
  <div class="kanban-column flex-shrink-0">
    <div :class="['card', 'h-100', `border-${column.color}`, 'border-opacity-25', 'bg-transparent']">
      <div :class="['card-header', 'column-header', 'd-flex', 'justify-content-between', 'align-items-center', 'border-0']">
        <div :class="['column-title-static', `text-${column.color}`, 'fw-bold', 'text-truncate']">
          {{ column.title }}
        </div>
        <button 
          class="btn btn-sm btn-outline-secondary opacity-50 text-muted shadow-none no-focus-ring"
          type="button"
          data-bs-toggle="modal"
          data-bs-target="#columnEditModal"
          @mousedown.stop
          @click="emit('editColumn', column)"
        >
          <i class="ti ti-pencil"></i>
        </button>
      </div>
      
      <div class="card-body p-2 kanban-column-body d-flex flex-column">
        <draggable 
          v-model="cards" 
          item-key="id"
          group="kanban-cards"
          class="card-list d-flex flex-column gap-2 flex-grow-1"
          ghost-class="ghost-card"
          handle=".drag-handle"
          :animation="200"
          :force-fallback="true"
          :disabled="!!searchQuery"
          :fallback-tolerance="3"
          @change="onDragChange"
        >
          <template #item="{ element: card }">
            <KanbanCardComp 
              v-if="filteredCards.find(c => c.id === card.id)"
              :card="card" 
              :column-id="column.id" 
              :color-options="colorOptions"
              :projects="projects"
              @remove="(colId, cardId) => emit('removeCard', colId, cardId)"
              @update="handleUpdateCard"
              @edit="(c) => emit('editCard', column.id, c)"
            />
          </template>
        </draggable>
        <div v-if="!searchQuery" class="d-flex gap-2 mt-2">
          <button class="btn btn-secondary btn-sm flex-grow-1 d-flex align-items-center justify-content-center gap-1" @click="emit('addCard', column.id)">
            <i class="ti ti-plus"></i> Add Card
          </button>
          <div class="dropdown" v-if="templates && templates.length > 0">
            <button class="btn btn-secondary btn-sm d-flex align-items-center justify-content-center gap-1 dropdown-toggle no-caret" type="button" data-bs-toggle="dropdown" aria-expanded="false">
              <i class="ti ti-template"></i>
            </button>
            <ul class="dropdown-menu dropdown-menu-end shadow-sm">
              <li><h6 class="dropdown-header">Use Template</h6></li>
              <li v-for="template in templates" :key="template.id" class="d-flex align-items-center pe-2">
                <a class="dropdown-item flex-grow-1" href="#" @click.prevent="emit('useTemplate', column.id, template)" data-bs-dismiss="dropdown">
                  {{ template.title }}
                </a>
                <button class="btn btn-xs text-danger p-0" @click.stop="emit('deleteTemplate', template.id)">
                  <i class="ti ti-trash"></i>
                </button>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.kanban-column {
  width: 300px;
  height: 100%;
}
.kanban-column-body {
  overflow-y: auto;
  overflow-x: hidden;
}
.column-header {
  cursor: grab;
}
.column-header:active {
  cursor: grabbing;
}
.column-title-static {
  font-weight: bold;
  width: 80%;
  padding: 0.375rem 0;
}
.card-list {
  min-height: 150px;
  overflow: visible;
}
.ghost-card {
  opacity: 0.5;
  background: #c8ebfb;
}
.color-box {
  width: 20px;
  height: 20px;
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid rgba(0,0,0,0.1);
}
.color-box.active {
  border: 2px solid #fff;
  box-shadow: 0 0 0 1px #000;
}
.dropdown-menu {
  z-index: 1050;
}
.dropdown-menu.show {
  display: block;
}
.no-focus-ring:focus {
  outline: none !important;
  box-shadow: none !important;
}
.btn-xs {
  padding: 1px 5px;
  font-size: 0.75rem;
}
.dropdown-toggle::after {
  display: none !important;
}
</style>
