<script setup lang="ts">
import draggable from 'vuedraggable';
import KanbanCardComp from './KanbanCard.vue';

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

const props = defineProps<{
  column: KanbanColumn;
  colorOptions: string[];
}>();

const emit = defineEmits<{
  (e: 'remove', id: string): void;
  (e: 'addCard', columnId: string): void;
  (e: 'removeCard', columnId: string, cardId: string): void;
  (e: 'updateCard', columnId: string, card: KanbanCard): void;
}>();

const handleUpdateCard = (card: KanbanCard) => {
  emit('updateCard', props.column.id, card);
};
</script>

<template>
  <div class="kanban-column flex-shrink-0">
    <div :class="['card', 'h-100', `border-${column.color}`, 'border-opacity-25', 'bg-transparent']">
      <div :class="['card-header', 'column-header', 'd-flex', 'justify-content-between', 'align-items-center', 'border-0']">
        <input 
          v-model="column.title" 
          :class="['column-title-input', `text-${column.color}`, 'form-control', 'bg-transparent','border-0']"
          @click.stop 
        />
        <div class="dropdown">
          <button class="btn btn-sm btn-link p-0" data-bs-toggle="dropdown" @click.stop>
            <i class="ti ti-dots-vertical"></i>
          </button>
          <ul class="dropdown-menu dropdown-menu-end">
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
            <li><a class="dropdown-item text-danger" href="#" @click.prevent="emit('remove', column.id)"><i class="ti ti-trash"></i> Remove Column</a></li>
          </ul>
        </div>
      </div>
      
      <div class="card-body p-2 kanban-column-body overflow-auto">
        <draggable
          v-model="column.cards"
          item-key="id"
          group="cards"
          ghost-class="ghost-card"
          animation="200"
          class="card-list d-flex flex-column gap-2"
          tag="div"
          :force-fallback="true"
          draggable=".kanban-card"
          :fallback-tolerance="3"
        >
          <template #item="{ element: card }">
            <KanbanCardComp 
              :card="card" 
              :column-id="column.id" 
              :color-options="colorOptions"
              @remove="(colId, cardId) => emit('removeCard', colId, cardId)"
              @update="handleUpdateCard"
            />
          </template>
        </draggable>
        <button class="btn btn-secondary btn-sm w-100 mt-2" @click="emit('addCard', column.id)">
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
}
.card-list {
  min-height: 50px;
}
.ghost-card {
  opacity: 0.5;
  background: var(--bs-secondary-bg);
  border: 2px dashed var(--bs-primary);
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
</style>
