<script setup lang="ts">
const emit = defineEmits(['add']);

const availableWidgets = [
  { id: 'countdown', name: 'Countdown', icon: 'ti-hourglass-high' },
  { id: 'clock', name: 'Clock', icon: 'ti-clock' },
  { id: 'placeholder1', name: 'Upcoming Tool 1', icon: 'ti-dots', disabled: true },
  { id: 'placeholder2', name: 'Upcoming Tool 2', icon: 'ti-dots', disabled: true },
];

const selectWidget = (type: string) => {
  emit('add', type);
};

</script>

<template>
  <div class="modal fade" id="addWidgetModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">Add Widget</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <div class="list-group">
            <button 
              v-for="widget in availableWidgets" 
              :key="widget.id"
              type="button" 
              class="list-group-item list-group-item-action d-flex align-items-center gap-3"
              :disabled="widget.disabled"
              @click="selectWidget(widget.id)"
              data-bs-dismiss="modal"
            >
              <i :class="['ti', widget.icon, 'fs-4']"></i>
              <span>{{ widget.name }}</span>
              <span v-if="widget.disabled" class="badge bg-secondary ms-auto">Coming Soon</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
