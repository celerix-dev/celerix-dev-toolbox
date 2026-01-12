<script setup lang="ts">
import { ref, computed } from 'vue';

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

const props = defineProps<{
  card: KanbanCard;
  columnId: string;
  colorOptions: string[];
  projects: Project[];
}>();

const emit = defineEmits<{
  (e: 'remove', columnId: string, cardId: string): void;
  (e: 'update', card: KanbanCard): void;
  (e: 'edit', card: KanbanCard): void;
}>();

const selectedProject = computed(() => {
  return props.projects.find(p => p.id === props.card.projectId);
});

const priorityColors = {
  low: 'success',
  medium: 'info',
  high: 'warning',
  urgent: 'danger'
};

const priorityIcons = {
  low: 'ti-arrow-down',
  medium: 'ti-minus',
  high: 'ti-arrow-up',
  urgent: 'ti-alert-circle'
};

const isOverdue = computed(() => {
  if (!props.card.dueDate) return false;
  return new Date(props.card.dueDate) < new Date();
});

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
};

const updateColor = (color: string) => {
  emit('update', { ...props.card, color });
};

const handleCardClick = (event: MouseEvent) => {
  // Don't open edit modal if we clicked on the dropdown or its items
  const target = event.target as HTMLElement;
  if (target.closest('.dropdown')) {
    return;
  }
  emit('edit', props.card);
};
</script>

<template>
  <div class="kanban-card">
    <div 
      :class="['card', `bg-${card.color}-subtle`, 'drag-handle']"
      @click="handleCardClick"
    >
      <div class="card-body p-2 d-flex flex-column h-100">
        <div class="d-flex justify-content-between align-items-start">
          <h6 class="card-title mb-1 text-truncate pe-4" :title="card.title">{{ card.title }}</h6>
          <div class="dropdown">
            <button 
              class="btn btn-sm btn-link p-0 shadow-none border-0 no-focus-ring dropdown-toggle" 
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
              <li><a class="dropdown-item" href="#" @click.prevent="emit('edit', card)"><i class="ti ti-edit"></i> Edit Card</a></li>
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
        
        <div class="mt-2 d-flex flex-wrap gap-1 align-items-center">
          <!-- Project Tag -->
          <span v-if="selectedProject" class="badge" :style="{ backgroundColor: selectedProject.color + '20', color: selectedProject.color }">
            <i :class="['ti', selectedProject.icon, 'me-1']"></i>
            #{{ selectedProject.tag }}
          </span>

          <!-- Priority -->
          <span v-if="card.priority" :class="['badge', `text-${priorityColors[card.priority]}`, `bg-${priorityColors[card.priority]}-subtle`] " :title="`Priority: ${card.priority}`">
            <i :class="['ti', priorityIcons[card.priority], 'me-1']"></i>
            {{ card.priority }}
          </span>

          <!-- Due Date -->
          <span v-if="card.dueDate" :class="['badge', isOverdue ? 'text-danger bg-danger-subtle' : 'text-muted bg-light-subtle']" :title="`Due Date: ${card.dueDate}`">
            <i class="ti ti-calendar-event me-1"></i>
            {{ formatDate(card.dueDate) }}
          </span>
        </div>

        <div class="mt-2 d-flex justify-content-between align-items-center">
           <div class="text-muted smaller opacity-50">
             <i class="ti ti-user me-1"></i> {{ card.assignee || 'Unassigned' }}
           </div>
           <button class="btn btn-xs btn-outline-secondary opacity-50" @click.stop="emit('edit', card)" @mousedown.stop>
             <i class="ti ti-pencil"></i>
           </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.smaller {
  font-size: 0.75rem;
}

.drag-handle {
  cursor: grab;
}

.drag-handle:active {
  cursor: grabbing;
}

.kanban-card {
  margin-bottom: 0.5rem;
  width: 100%;
}

.kanban-card:hover .card {
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
  transform: translateY(-2px);
}

.card {
  transition: all 0.2s ease;
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
