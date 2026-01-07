<script setup lang="ts">
import { ref } from 'vue';

interface KanbanCard {
  id: string;
  title: string;
  description: string;
  color?: string;
}

const props = defineProps<{
  card: KanbanCard;
  columnId: string;
  colorOptions: string[];
}>();

const emit = defineEmits<{
  (e: 'remove', columnId: string, cardId: string): void;
  (e: 'update', card: KanbanCard): void;
}>();

const isFlipped = ref(false);

const toggleFlip = () => {
  isFlipped.value = !isFlipped.value;
};

const onDone = () => {
  isFlipped.value = false;
};

const updateColor = (color: string) => {
  emit('update', { ...props.card, color });
};
</script>

<template>
  <div :class="['kanban-card', { 'flipped': isFlipped }]">
    <!-- Front Side -->
    <div 
      v-if="!isFlipped"
      :class="['card', `bg-${card.color}-subtle`]"
    >
      <div class="card-body p-2 d-flex flex-column h-100">
        <div class="d-flex justify-content-between align-items-start">
          <h6 class="card-title mb-1 text-truncate pe-4" :title="card.title">{{ card.title }}</h6>
          <div class="dropdown">
            <button class="btn btn-sm btn-link p-0" data-bs-toggle="dropdown" @click.stop>
              <i class="ti ti-dots-vertical"></i>
            </button>
            <ul class="dropdown-menu dropdown-menu-end">
              <li><a class="dropdown-item" href="#" @click.prevent="toggleFlip"><i class="ti ti-edit"></i> Edit Card</a></li>
              <li><h6 class="dropdown-header">Card Color</h6></li>
              <li class="px-2 d-flex flex-wrap gap-1 mb-2" style="max-width: 150px;">
                <div 
                  v-for="color in colorOptions" 
                  :key="color"
                  :class="['color-box', `bg-${color}`, card.color === color ? 'active' : '']"
                  @click="updateColor(color)"
                ></div>
              </li>
              <li><hr class="dropdown-divider"></li>
              <li><a class="dropdown-item text-danger" href="#" @click.prevent="emit('remove', columnId, card.id)"><i class="ti ti-trash"></i> Remove Card</a></li>
            </ul>
          </div>
        </div>
        <p class="card-text small mb-0 flex-grow-1 text-muted-custom">{{ card.description || 'No description' }}</p>
        <div class="mt-2 text-end">
           <button class="btn btn-xs btn-outline-secondary opacity-50" @click="toggleFlip">
             <i class="ti ti-pencil"></i>
           </button>
        </div>
      </div>
    </div>

    <!-- Back Side (Edit Mode) -->
    <div 
      v-else
      :class="['card', `bg-${card.color}-subtle`]"
    >
      <div class="card-body p-2 d-flex flex-column h-100">
        <div class="mb-2">
          <label class="form-label small mb-1">Title</label>
          <input v-model="card.title" class="form-control form-control-sm bg-transparent border-secondary" />
        </div>
        <div class="mb-2 flex-grow-1">
          <label class="form-label small mb-1">Description</label>
          <textarea v-model="card.description" class="form-control form-control-sm bg-transparent border-secondary" rows="3" placeholder="Description..."></textarea>
        </div>
        <button class="btn btn-primary btn-sm w-100" @click="onDone">Done</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.kanban-card {
  cursor: grab;
  margin-bottom: 0.5rem;
  width: 100%;
}

.kanban-card:active {
  cursor: grabbing;
}

.text-muted-custom {
  opacity: 0.8;
}

.btn-xs {
  padding: 1px 5px;
  font-size: 0.75rem;
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
