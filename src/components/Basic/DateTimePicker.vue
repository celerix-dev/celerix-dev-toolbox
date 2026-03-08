<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import dayjs from 'dayjs';

interface Props {
    modelValue: string | null;
}

const props = defineProps<Props>();
const emit = defineEmits(['update:modelValue']);

// Controls which month we are looking at (defaults to props or today)
const currentViewDate = ref(dayjs(props.modelValue || undefined));

// Internal Time State - synced with modelValue
const hours = ref(props.modelValue ? dayjs(props.modelValue).hour() : 0);
const minutes = ref(props.modelValue ? dayjs(props.modelValue).minute() : 0);
const seconds = ref(props.modelValue ? dayjs(props.modelValue).second() : 0);

const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

const daysInMonth = computed(() => {
    const startOfMonth = currentViewDate.value.startOf('month');
    const endOfMonth = currentViewDate.value.endOf('month');
    const days = [];

    for (let i = 0; i < startOfMonth.day(); i++) days.push(null);
    for (let i = 1; i <= endOfMonth.date(); i++) days.push(startOfMonth.date(i));

    return days;
});

const selectDay = (date: dayjs.Dayjs) => {
    const final = date.hour(hours.value).minute(minutes.value).second(seconds.value);
    emit('update:modelValue', final.format('YYYY-MM-DDTHH:mm:ss'));
};

const changeMonth = (offset: number) => {
    currentViewDate.value = currentViewDate.value.add(offset, 'month');
};

// Sync time changes
watch([hours, minutes, seconds], () => {
    if (props.modelValue) {
        const current = dayjs(props.modelValue).hour(hours.value).minute(minutes.value).second(seconds.value);
        emit('update:modelValue', current.format('YYYY-MM-DDTHH:mm:ss'));
    }
});
</script>

<template>
    <div class="date-time-picker-ui card">
        <div class="card-body p-2">
            <div class="d-flex justify-between align-center mb-2">
                <button type="button" class="cx-button small" @click="changeMonth(-1)">
                    <i class="ti ti-chevron-left"></i>
                </button>
                <div class="fw-bold">{{ currentViewDate.format('MMMM YYYY') }}</div>
                <button type="button" class="cx-button small" @click="changeMonth(1)">
                    <i class="ti ti-chevron-right"></i>
                </button>
            </div>

            <div class="calendar-grid mb-3">
                <div v-for="day in weekdays" :key="day" class="weekday text-muted">{{ day }}</div>
                <div v-for="(date, index) in daysInMonth" :key="index"
                     class="calendar-day"
                     :class="{
                        'empty': !date,
                        'selected': date && modelValue && date.isSame(dayjs(modelValue), 'day'),
                        'today': date && date.isSame(dayjs(), 'day')
                    }"
                     @click="date && selectDay(date)">
                    {{ date ? date.date() : '' }}
                </div>
            </div>

            <div class="time-picker-controls border-top pt-2 d-flex g-1 justify-center">
                <div class="time-unit">
                    <label>Hr</label>
                    <input type="number" v-model.number="hours" min="0" max="23" class="cx-input text-center">
                </div>
                <div class="time-unit">
                    <label>Min</label>
                    <input type="number" v-model.number="minutes" min="0" max="59" class="cx-input text-center">
                </div>
                <div class="time-unit">
                    <label>Sec</label>
                    <input type="number" v-model.number="seconds" min="0" max="59" class="cx-input text-center">
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* No absolute positioning or z-index here! */
.date-time-picker-ui {
    width: 100%;
    max-width: 280px;
    background: var(--bg-surface);
}
.calendar-grid { display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; }
.calendar-day { text-align: center; padding: 8px 0; cursor: pointer; border-radius: 4px; }
.calendar-day.selected { background: var(--brand); color: white; }
.calendar-day.today { border: 1px solid var(--brand); }
</style>
