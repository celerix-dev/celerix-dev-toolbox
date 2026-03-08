<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { openModal, closeModal } from '@celerix/spectrum/ui';

const props = defineProps<{
    show: boolean;
    title: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
    variant?: 'primary' | 'danger' | 'warning' | 'info';
    loading?: boolean;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'confirm'): void;
}>();

const modalRef = ref<HTMLElement | null>(null);

// Bridge Vue 'show' prop to the Library's DOM engine
watch(() => props.show, (shouldShow) => {
    if (!modalRef.value) return;
    const isCurrentlyShow = modalRef.value.classList.contains('show');

    if (shouldShow && !isCurrentlyShow) openModal(modalRef.value);
    if (!shouldShow && isCurrentlyShow) closeModal(modalRef.value);
});

const handleClosedEvent = () => {
    // If the DOM closed (backdrop/esc), notify the parent to update the 'show' prop
    if (props.show) {
        emit('close');
    }
};

onMounted(() => {
    modalRef.value?.addEventListener('cx-modal-closed', handleClosedEvent);
});

onUnmounted(() => {
    modalRef.value?.removeEventListener('cx-modal-closed', handleClosedEvent);
});
</script>

<template>
    <div
        class="cx-modal"
        ref="modalRef"
        data-cx-modal
    >
        <div class="cx-modal-dialog">
            <div class="cx-modal-content">
                <div class="cx-modal-header">
                    <h5 class="cx-modal-title">{{ title }}</h5>
                    <button
                        type="button"
                        class="cx-button small ti ti-x"
                        @click="emit('close')"
                        :disabled="loading"
                    ></button>
                </div>

                <div class="cx-modal-body">
                    <p class="m-0">{{ message }}</p>
                </div>

                <div class="cx-modal-footer">
                    <div class="cx-button-group">
                        <button
                            type="button"
                            class="cx-button"
                            @click="emit('close')"
                            :disabled="loading"
                        >
                            <span>{{ cancelText || 'Cancel' }}</span>
                        </button>
                        <button
                            type="button"
                            :class="['cx-button', variant ? `is-${variant}` : 'is-brand']"
                            @click="emit('confirm')"
                            :disabled="loading"
                        >
                            <i v-if="loading" class="ti ti-loader-2 animate-spin me-s-1"></i>
                            <span>{{ confirmText || 'Confirm' }}</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
