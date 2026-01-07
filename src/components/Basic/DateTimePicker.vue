<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import dayjs from 'dayjs';

interface Props {
  modelValue: string;
}

const props = defineProps<Props>();
const emit = defineEmits(['update:modelValue']);

const showPicker = ref(false);
const currentViewDate = ref(dayjs(props.modelValue || undefined));
const selectedDate = ref(props.modelValue ? dayjs(props.modelValue) : null);

// Time parts
const hours = ref(selectedDate.value ? selectedDate.value.hour() : 0);
const minutes = ref(selectedDate.value ? selectedDate.value.minute() : 0);
const seconds = ref(selectedDate.value ? selectedDate.value.second() : 0);

const daysInMonth = computed(() => {
  const startOfMonth = currentViewDate.value.startOf('month');
  const endOfMonth = currentViewDate.value.endOf('month');
  const days = [];
  
  // Padding for start of month (Sunday = 0, Monday = 1, etc.)
  const startDay = startOfMonth.day();
  for (let i = 0; i < startDay; i++) {
    days.push(null);
  }
  
  for (let i = 1; i <= endOfMonth.date(); i++) {
    days.push(startOfMonth.date(i));
  }
  
  return days;
});

const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

const prevMonth = () => {
  currentViewDate.value = currentViewDate.value.subtract(1, 'month');
};

const nextMonth = () => {
  currentViewDate.value = currentViewDate.value.add(1, 'month');
};

const selectDay = (date: dayjs.Dayjs) => {
  selectedDate.value = date.hour(hours.value).minute(minutes.value).second(seconds.value);
  updateValue();
};

const updateValue = () => {
  if (selectedDate.value) {
    const finalDate = selectedDate.value.hour(hours.value).minute(minutes.value).second(seconds.value);
    emit('update:modelValue', finalDate.format('YYYY-MM-DDTHH:mm:ss'));
  }
};

watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    const d = dayjs(newVal);
    selectedDate.value = d;
    hours.value = d.hour();
    minutes.value = d.minute();
    seconds.value = d.second();
  }
});

watch([hours, minutes, seconds], updateValue);

const isSelected = (date: dayjs.Dayjs | null) => {
  if (!date || !selectedDate.value) return false;
  return date.isSame(selectedDate.value, 'day');
};

const isToday = (date: dayjs.Dayjs | null) => {
  if (!date) return false;
  return date.isSame(dayjs(), 'day');
};

const formattedValue = computed(() => {
  if (!props.modelValue) return 'Select Date & Time';
  return dayjs(props.modelValue).format('YYYY-MM-DD HH:mm:ss');
});

const togglePicker = () => {
  showPicker.value = !showPicker.value;
};

// Close picker when clicking outside
const pickerContainer = ref<HTMLElement | null>(null);
const handleClickOutside = (event: MouseEvent) => {
  if (pickerContainer.value && !pickerContainer.value.contains(event.target as Node)) {
    showPicker.value = false;
  }
};

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside);
});

</script>

<template>
  <div class="date-time-picker position-relative" ref="pickerContainer">
    <div class="input-group cursor-pointer" @click="togglePicker">
      <span class="input-group-text"><i class="ti ti-calendar"></i></span>
      <input 
        type="text" 
        class="form-control" 
        :value="formattedValue" 
        readonly
        placeholder="Select Date & Time"
      >
    </div>

    <div v-if="showPicker" class="picker-dropdown card shadow mt-1">
      <div class="card-body p-2">
        <!-- Month Navigation -->
        <div class="d-flex justify-content-between align-items-center mb-2">
          <button class="btn btn-sm btn-light" @click="prevMonth"><i class="ti ti-chevron-left"></i></button>
          <div class="fw-bold">{{ currentViewDate.format('MMMM YYYY') }}</div>
          <button class="btn btn-sm btn-light" @click="nextMonth"><i class="ti ti-chevron-right"></i></button>
        </div>

        <!-- Calendar Grid -->
        <div class="calendar-grid mb-3">
          <div v-for="day in weekdays" :key="day" class="weekday text-muted">{{ day }}</div>
          <div 
            v-for="(date, index) in daysInMonth" 
            :key="index" 
            class="calendar-day"
            :class="{ 
              'empty': !date, 
              'selected': isSelected(date),
              'today': isToday(date)
            }"
            @click="date && selectDay(date)"
          >
            {{ date ? date.date() : '' }}
          </div>
        </div>

        <!-- Time Picker -->
        <div class="time-picker-controls border-top pt-2 mt-2">
          <div class="row g-1 align-items-center">
            <div class="col">
              <label class="small text-muted d-block text-center">Hour</label>
              <input type="number" v-model.number="hours" min="0" max="23" class="form-control form-control-sm text-center">
            </div>
            <div class="col-auto pt-3">:</div>
            <div class="col">
              <label class="small text-muted d-block text-center">Min</label>
              <input type="number" v-model.number="minutes" min="0" max="59" class="form-control form-control-sm text-center">
            </div>
            <div class="col-auto pt-3">:</div>
            <div class="col">
              <label class="small text-muted d-block text-center">Sec</label>
              <input type="number" v-model.number="seconds" min="0" max="59" class="form-control form-control-sm text-center">
            </div>
          </div>
        </div>
        
        <div class="mt-3">
          <button class="btn btn-primary btn-sm w-100" @click="showPicker = false">Done</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.date-time-picker {
  z-index: 1000;
}
.picker-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  width: 280px;
  z-index: 1050;
  background-color: var(--bs-card-bg);
}
.calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
}
.weekday {
  text-align: center;
  font-size: 0.75rem;
  font-weight: bold;
  padding: 5px 0;
}
.calendar-day {
  text-align: center;
  padding: 5px 0;
  cursor: pointer;
  border-radius: 4px;
  font-size: 0.9rem;
}
.calendar-day:hover:not(.empty) {
  background-color: var(--bs-secondary-bg);
}
.calendar-day.selected {
  background-color: var(--bs-primary) !important;
  color: white;
}
.calendar-day.today {
  border: 1px solid var(--bs-primary);
  color: var(--bs-primary);
}
.calendar-day.selected.today {
  color: white;
}
.calendar-day.empty {
  cursor: default;
}
.cursor-pointer {
  cursor: pointer;
}
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
