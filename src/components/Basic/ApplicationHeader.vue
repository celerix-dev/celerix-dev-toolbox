<script setup lang="ts">
import { ref } from 'vue';
import ConfirmationModal from './ConfirmationModal.vue';
import { eventBus } from '@/services/events';

const showResetConfirm = ref(false);

const handleResetConfirm = () => {
    eventBus.emit('reset-dashboard');
    showResetConfirm.value = false;
};
</script>

<template>
    <div class="navbar navbar-expand-md docs-navbar sticky-top glass-bg justify-content-between p-3" style="height: 61px;z-index:9999">
        <div id="breadcrumbs" class="align-content-center ps-4 ps-md-0" style="height: 20px"></div>
        <div id="page-context"></div>
        <div class="d-flex">
            <div class="d-flex gap-1">
                <button class="btn btn-outline-info">
                    <span class="bg-color-light">
                        <i class="ti ti-camera-bolt"></i>
                    </span>
                </button>
                <div id="screenshotButton" class="donkers-button round large icon" aria-hidden="true"></div>

                <div class="dropdown">
                    <button class="btn btn-outline-info dropdown-toggle" style="min-width: 70px" type="button" data-bs-toggle="dropdown" aria-expanded="false">
                        <span>Options</span>
                    </button>

                    <ul class="dropdown-menu dropdown-menu-end">
                        <li><h6 class="dropdown-header">Your account</h6></li>
                        <li>
                            <RouterLink :to="{ name: 'profile' }" class="dropdown-item" aria-current="true"> <i class="ti ti-user-star"></i> Profile </RouterLink>
                        </li>
                        <li>
                            <RouterLink :to="{ name: 'settings' }" class="dropdown-item" aria-current="true"> <i class="ti ti-adjustments-cog"></i> Settings </RouterLink>
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
</template>

<style scoped></style>
