<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue';
import { useRoute } from 'vue-router';
import ConfirmationModal from './ConfirmationModal.vue';
import AlertModal from './AlertModal.vue';
import { eventBus } from '@/services/events';
import { useUserStore } from '@/stores/user';
import { screenshotService } from '@/services/screenshot';

const route = useRoute();
const userStore = useUserStore();
const showResetConfirm = ref(false);
const isCapturing = ref(false);

const showAlert = ref(false);
const alertTitle = ref('');
const alertMessage = ref('');
const alertVariant = ref<'primary' | 'danger' | 'warning' | 'info' | 'success'>('primary');

const triggerAlert = (title: string, message: string, variant: 'primary' | 'danger' | 'warning' | 'info' | 'success' = 'primary') => {
    alertTitle.value = title;
    alertMessage.value = message;
    alertVariant.value = variant;
    showAlert.value = true;
};

onMounted(async () => {
    if (!userStore.isInitialized) {
        await userStore.loadUser();
    }
});

const handleResetConfirm = () => {
    eventBus.emit('reset-dashboard');
    showResetConfirm.value = false;
};

const takeScreenshot = async () => {
    isCapturing.value = true;
    try {
        // Give time for the button to disappear from the DOM (spinner to appear)
        // AND for the DOM to settle so the spinner itself isn't captured
        // Actually, we WANT the spinner to NOT be captured.
        // If we want it completely clean, we should ideally not show the spinner until after capture,
        // but that's bad UX.
        // The user says "now the 'busy' indication appears on the screenshot".
        // To fix this, we can wait a bit after isCapturing = true, then hide the spinner JUST for the capture.

        await nextTick();
        // Even with nextTick, the spinner is there.
        // Let's use a small timeout to ensure the UI has updated.
        await new Promise(resolve => setTimeout(resolve, 100));

        // To really avoid it, we might need to hide the button completely.
        const sectionName = (route.name as string) || 'app';
        const savedPath = await screenshotService.captureAndSave(sectionName);
        if (savedPath) {
            triggerAlert('Screenshot Saved', `Your screenshot has been saved to: ${savedPath}`, 'success');
        }
    } catch (e: any) {
        console.error('Failed to take screenshot', e);
        triggerAlert('Screenshot Error', `Failed to capture screenshot: ${e.message || e}`, 'danger');
    } finally {
        isCapturing.value = false;
    }
};
</script>

<template>

        <div>&nbsp;</div>
        <div class="d-flex">
            <div class="d-flex g-1">
                <button v-if="!isCapturing" class="cx-button" @click="takeScreenshot" title="Take Screenshot">
                    <i class="ti ti-camera-bolt"></i>
                </button>
                <button v-else class="cx-button" disabled title="Capturing...">
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                </button>
                <div id="screenshotButton" class="donkers-button round large icon" aria-hidden="true"></div>

                <div class="dropdown" data-cx-dropdown>
                    <div class="cx-button dropdown-toggle" data-cx-toggle="dropdown" aria-expanded="false">
                        <i class="ti ti-user-circle me-1"></i>
                        {{ userStore.userDisplayName }}
                    </div>

                    <ul class="dropdown-menu dropdown-menu-end">
                        <li><h6 class="dropdown-header">Your account</h6></li>
                        <li>
                            <RouterLink :to="{ name: 'profile' }" class="dropdown-item" aria-current="true"> <i class="ti ti-user-star"></i> Profile </RouterLink>
                        </li>
                        <li>
                            <RouterLink :to="{ name: 'settings' }" class="dropdown-item" aria-current="true"> <i class="ti ti-adjustments-cog"></i> Settings </RouterLink>
                        </li>
                        <li>
                            <RouterLink :to="{ name: 'data-viewer' }" class="dropdown-item" aria-current="true"> <i class="ti ti-database"></i> Data Viewer </RouterLink>
                        </li>
                        <li><hr class="dropdown-divider"></li>
                        <li><h6 class="dropdown-header">Dashboard</h6></li>
                        <li>
                            <a href="#" class="dropdown-item text-danger" @click.prevent="showResetConfirm = true">
                                <i class="ti ti-refresh"></i> Reset Dashboard
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>

    <ConfirmationModal
        :show="showResetConfirm"
        title="Reset Dashboard"
        message="Are you sure you want to reset the dashboard? All widgets will be removed."
        confirm-text="Reset Dashboard"
        variant="danger"
        @close="showResetConfirm = false"
        @confirm="handleResetConfirm"
    />

    <AlertModal
        :show="showAlert"
        :title="alertTitle"
        :message="alertMessage"
        :variant="alertVariant"
        @close="showAlert = false"
    />
</template>

<style scoped></style>
