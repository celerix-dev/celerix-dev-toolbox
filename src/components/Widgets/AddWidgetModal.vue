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
    <div class="cx-modal" id="addWidgetModal" data-cx-modal>
        <div class="cx-modal-dialog">
            <div class="cx-modal-content">
                <div class="cx-modal-header">
                    <h5 class="cx-modal-title">Add Widget</h5>
                    <button type="button" class="cx-button small ti ti-x" data-cx-dismiss="modal"></button>
                </div>
                <div class="cx-modal-body">
                    <div class="cx-list-group d-flex-col">
                        <button
                            v-for="widget in availableWidgets"
                            :key="widget.id"
                            class="cx-list-item"
                            :disabled="widget.disabled"
                            @click="selectWidget(widget.id)"
                            data-cx-dismiss="modal"
                        >
                            <i :class="['ti', widget.icon]"></i>
                            <span>{{ widget.name }}</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
