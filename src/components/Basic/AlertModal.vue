<script setup lang="ts">

defineProps<{
  show: boolean;
  title: string;
  message: string;
  buttonText?: string;
  variant?: 'primary' | 'danger' | 'warning' | 'info' | 'success';
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();
</script>

<template>
  <div v-if="show" class="modal fade show d-block" tabindex="-1" style="background: rgba(0,0,0,0.5); z-index: 1070;">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content border-0 shadow">
        <div class="modal-header border-bottom-0 pb-0">
          <h5 class="modal-title fw-bold">{{ title }}</h5>
          <button type="button" class="btn-close" @click="emit('close')"></button>
        </div>
        <div class="modal-body py-4">
          <p class="mb-0">{{ message }}</p>
        </div>
        <div class="modal-footer border-top-0 pt-0">
          <button 
            type="button" 
            :class="['btn px-4', variant ? `btn-${variant}` : 'btn-primary']"
            @click="emit('close')"
          >
            {{ buttonText || 'OK' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
