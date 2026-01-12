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

const props = defineProps<{
  column: KanbanColumn;
  colorOptions: string[];
  projects: Project[];
}>();

const emit = defineEmits<{
  (e: 'remove', id: string): void;
  (e: 'addCard', columnId: string): void;
  (e: 'removeCard', columnId: string, cardId: string): void;
  (e: 'updateCard', columnId: string, card: KanbanCard): void;
  (e: 'editCard', columnId: string, card: KanbanCard): void;
  (e: 'update:column', column: KanbanColumn): void;
}>();

const cards = computed({
  get: () => props.column.cards,
  set: (newCards) => {
    emit('update:column', { ...props.column, cards: newCards });
  }
});

const handleUpdateCard = (card: KanbanCard) => {
  emit('updateCard', props.column.id, card);
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

  <div class="kanban-column flex-shrink-0">
    <div :class="['card', 'h-100', `border-${column.color}`, 'border-opacity-25', 'bg-transparent']">
      <div :class="['card-header', 'column-header', 'd-flex', 'justify-content-between', 'align-items-center', 'border-0']">
        <input 
          v-model="column.title" 
          :class="['column-title-input', `text-${column.color}`, 'form-control', 'bg-transparent','border-0']"
        />
        <div class="dropdown">
          <button 
            class="btn btn-sm btn-link p-0 text-muted shadow-none border-0 no-focus-ring dropdown-toggle" 
            type="button"
            data-bs-toggle="dropdown" 
            data-bs-display="static"
            data-bs-auto-close="outside"
            aria-expanded="false"
            @mousedown.stop
          >
            <i class="ti ti-dots-vertical"></i>
          </button>
          <ul class="dropdown-menu dropdown-menu-end shadow-sm">
            <li><h6 class="dropdown-header">Header Color</h6></li>
            <li class="px-2 d-flex flex-wrap gap-1 mb-2" style="max-width: 150px;">
              <div 
                v-for="color in colorOptions" 
                :key="color"
                :class="['color-box', `bg-${color}`, column.color === color ? 'active' : '']"
                @click="column.color = color"
              ></div>
            </li>
            <li><hr class="dropdown-divider"></li>
            <li>
              <a class="dropdown-item text-danger d-flex align-items-center gap-2" href="#" @click.prevent="emit('remove', column.id)">
                <i class="ti ti-trash"></i> Remove Column
              </a>
            </li>
          </ul>
        </div>
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
          :fallback-tolerance="3"
        >
          <template #item="{ element: card }">
            <KanbanCardComp 
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
        <button class="btn btn-secondary btn-sm w-100 mt-2 d-flex align-items-center justify-content-center gap-1" @click="emit('addCard', column.id)">
          <i class="ti ti-plus"></i> Add Card
        </button>
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
.column-title-input {
  background: transparent;
  border: none;
  font-weight: bold;
  width: 80%;
  outline: none;
  cursor: text;
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
.dropdown-toggle::after {
  display: none !important;
}
</style>
