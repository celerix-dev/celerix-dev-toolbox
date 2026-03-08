<script setup lang="ts">

import ClockWidget from "@/components/Widgets/ClockWidget.vue";
import CountdownWidget from "@/components/Widgets/CountdownWidget.vue";
import {onBeforeUnmount, onMounted, ref, watch} from "vue";
import AlertModal from '@/components/Basic/AlertModal.vue';
import {open, save} from '@tauri-apps/plugin-dialog';
import {readTextFile, writeTextFile} from '@tauri-apps/plugin-fs';
import draggable from "vuedraggable";
import {eventBus} from "@/services/events";
import {storageService} from "@/services/storage";
import {useUserStore} from "@/stores/user";
import AddWidgetModal from "@/components/Widgets/AddWidgetModal.vue";

const userStore = useUserStore();

interface Widget {
    id: string;
    type: 'countdown' | 'clock';
    label: string;
    targetDate?: string;
    isConfigured: boolean;
    clockOptions?: {
        showDate: boolean;
        showDigital: boolean;
        show24h: boolean;
    };
    countdownItems?: {
        id: string;
        label: string;
        targetDate: string;
    }[];
}

const widgets = ref<Widget[]>([]);
const isInitialized = ref(false);

// Alert Modal state
const showAlert = ref(false);
const alertTitle = ref('');
const alertMessage = ref('');

const triggerAlert = (title: string, message: string) => {
    alertTitle.value = title;
    alertMessage.value = message;
    showAlert.value = true;
};

const loadWidgets = async () => {
    try {
        const saved = await storageService.load<Widget[]>('WIDGETS');
        if (saved) {
            widgets.value = saved;
        }

        // Ensure the clock widget exists if a list is empty
        if (widgets.value.length === 0) {
            widgets.value.push({
                id: 'default-clock',
                type: 'clock',
                label: 'Welcome',
                isConfigured: true,
                clockOptions: {
                    showDate: false,
                    showDigital: true,
                    show24h: false
                }
            });
        }
    } catch (e) {
        console.error('Error loading widgets', e);
        throw e;
    }
};

const saveWidgets = async () => {
    if (!isInitialized.value) return;
    await storageService.save('WIDGETS', widgets.value);
};

watch(widgets, saveWidgets, {deep: true});

onMounted(async () => {
    // colorScheme.updateTheme();
    console.log('Dashboard: Starting initialization...');
    try {
        await Promise.all([
            loadWidgets(),
            userStore.loadUser()
        ]);
        isInitialized.value = true;
        console.log('Dashboard: Initialization successful.');
    } catch (e: any) {
        console.error('Dashboard: Failed to initialize', e);
        triggerAlert('Load Error', 'Failed to load dashboard widgets. Auto-save disabled to prevent data loss. Error: ' + e.message);
    }
    eventBus.on('reset-dashboard', handleReset);

    //
    // const canvas = document.getElementById('static-canvas');
    // const ctx = canvas.getContext('2d');
    //
    // function resize() {
    //     canvas.width = window.innerWidth / 2; // Lower resolution = chunkier, more authentic static
    //     canvas.height = window.innerHeight / 2;
    // }
    //
    // function snow() {
    //     const w = canvas.width;
    //     const h = canvas.height;
    //     const imageData = ctx.createImageData(w, h);
    //     const data = imageData.data;
    //
    //     for (let i = 0; i < data.length; i += 4) {
    //         // Generate a random gray value
    //         const noise = Math.random() * 255;
    //         data[i]     = noise; // Red
    //         data[i + 1] = noise; // Green
    //         data[i + 2] = noise; // Blue
    //         data[i + 3] = 255;   // Alpha (Opacity)
    //     }
    //
    //     ctx.putImageData(imageData, 0, 0);
    //     requestAnimationFrame(snow); // This creates the 60fps flicker
    // }
    //
    // window.addEventListener('resize', resize);
    // resize();
    // snow();
});

onBeforeUnmount(() => {
    eventBus.off('reset-dashboard', handleReset);
});

const handleReset = async () => {
    // We can't easily "remove" from storage service yet without a delete method,
    // but clearing the array and saving accomplishes the same for the UI.
    widgets.value = [];
    await saveWidgets();
    await loadWidgets();
};

const addWidget = (type: string) => {
    if (type === 'countdown') {
        widgets.value.push({
            id: crypto.randomUUID(),
            type: 'countdown',
            label: 'Countdown',
            isConfigured: false,
            countdownItems: []
        });
    } else if (type === 'clock') {
        widgets.value.push({
            id: crypto.randomUUID(),
            type: 'clock',
            label: 'Clock',
            isConfigured: false,
            clockOptions: {
                showDate: false,
                showDigital: true,
                show24h: false
            }
        });
    }
};

const updateWidget = (updatedWidget: Widget) => {
    const index = widgets.value.findIndex(w => w.id === updatedWidget.id);
    if (index !== -1) {
        widgets.value[index] = updatedWidget;
    }
};

const removeWidget = (id: string) => {
    widgets.value = widgets.value.filter(w => w.id !== id);
};

const exportWidgets = async () => {
    try {
        const filePath = await save({
            filters: [{
                name: 'JSON',
                extensions: ['json']
            }],
            defaultPath: 'celerix-widgets.json'
        });

        if (filePath) {
            await writeTextFile(filePath, JSON.stringify(widgets.value, null, 2));
        }
    } catch (e) {
        console.error('Failed to export widgets', e);
    }
};

const importWidgets = async () => {
    try {
        const selected = await open({
            multiple: false,
            filters: [{
                name: 'JSON',
                extensions: ['json']
            }]
        });

        if (selected) {
            const content = await readTextFile(selected);
            const imported = JSON.parse(content);

            // Basic validation
            if (Array.isArray(imported)) {
                // We could do more thorough validation here if needed
                widgets.value = imported;
            }
        }
    } catch (e) {
        console.error('Failed to import widgets', e);
    }
};


</script>

<template>
    <teleport to="#breadcrumbs">
        <div class="d-flex align-center justify-between cx-w-100">
            <div class="d-flex align-center g-2">
                <i class="ti ti-dashboard"></i>
                <strong>Dashboard</strong>
            </div>
        </div>
    </teleport>

    <teleport to="#page-context">
        <div class="cx-button-group">
            <button class="cx-button outlined g-2" @click="importWidgets" title="Import Widgets">
                <i class="ti ti-download"></i> Import
            </button>
            <button class="cx-button outlined g-2" @click="exportWidgets" title="Export Widgets">
                <i class="ti ti-upload"></i> Export
            </button>
        </div>
    </teleport>

<!--    <div class="static">-->
<!--        <canvas id="static-canvas"></canvas>-->
<!--    </div>-->

    <div class="p-2">
        <draggable
            v-model="widgets"
            item-key="id"
            tag="div"
            class="cx-grid-300 g-3"
            handle=".card-header"
            ghost-class="ghost-widget"
            animation="200"
            :force-fallback="true"
            draggable=".widget-col"
            :fallback-tolerance="3"
        >
            <template #item="{ element: widget }">
                <div class="widget-col">
                    <ClockWidget
                        v-if="widget.type === 'clock'"
                        :widget="widget"
                        @update="updateWidget"
                        @remove="removeWidget"
                    />
                    <CountdownWidget
                        v-else-if="widget.type === 'countdown'"
                        :widget="widget"
                        @update="updateWidget"
                        @remove="removeWidget"
                    />
                    <div v-else class="card base border-danger h-100 widget-card">
                        <div class="card-header bg-danger text-white d-flex justify-between align-center">
                            <span><i class="ti ti-alert-triangle"></i> Unknown Widget</span>
                            <button class="cx-button btn-sm text-white p-0" @click="removeWidget(widget.id)">
                                <i class="ti ti-trash"></i>
                            </button>
                        </div>
                        <div class="card-body d-flex flex-dir-col justify-center align-center text-center">
                            <p class="mb-0 text-danger">The widget type <strong>"{{ widget.type }}"</strong> is not
                                recognized.</p>
                            <small class="text-muted mt-2">ID: {{ widget.id }}</small>
                        </div>
                    </div>
                </div>
            </template>

            <template #footer>
                <!-- Add Widget Button -->
                <div class="">
                    <div class="card base d-flex add-widget-card cx-h-100" data-cx-toggle="modal" data-cx-target="#addWidgetModal">
                        <div class="card-body d-flex-col justify-center align-center text-muted flex-grow">
                            <i class="ti ti-plus fs-1"></i>
                            <span>Add Widget</span>
                        </div>
                    </div>
                </div>
            </template>
        </draggable>
    </div>

    <AlertModal
        :show="showAlert"
        :title="alertTitle"
        :message="alertMessage"
        variant="danger"
        @close="showAlert = false"
    />

    <AddWidgetModal @add="addWidget"/>

</template>

<style scoped>
/* #static-canvas {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    opacity: 0.15; /* Turn this up for more intense "snow"
    filter: contrast(180%) brightness(120%);
    animation: pulse 0.05s infinite;
}*/

.add-widget-card {
    box-shadow: none;
    border-style: dashed;

    &:hover {
        background-color: var(--bg-surface);
    }
}

.ghost-widget {
    opacity: 0.5;
    background: var(--bs-secondary-bg);
    border: 2px dashed var(--bs-primary);
}

:deep(.card-header) {
    cursor: grab;
}

:deep(.card-header:active) {
    cursor: grabbing;
}

</style>
