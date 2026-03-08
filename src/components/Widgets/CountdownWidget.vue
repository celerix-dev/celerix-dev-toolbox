<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue';
import dayjs from 'dayjs';
import duration from 'dayjs/plugin/duration';
import DateTimePicker from '@/components/Basic/DateTimePicker.vue';

dayjs.extend(duration);

interface CountdownItem {
    id: string;
    label: string;
    targetDate: string;
}

interface Props {
    widget: {
        id: string;
        label: string;
        isConfigured: boolean;
        type: 'countdown' | 'clock';
        countdownItems?: CountdownItem[];
        // Legacy support for single countdown
        targetDate?: string;
    };
}

const props = defineProps<Props>();
const emit = defineEmits(['update', 'remove']);

const editWidgetTitle = ref(props.widget.label || 'Countdown');
const editItems = ref<CountdownItem[]>([]);

const initEditMode = () => {
    editWidgetTitle.value = props.widget.label || 'Countdown';
    if (props.widget.countdownItems && props.widget.countdownItems.length > 0) {
        editItems.value = props.widget.countdownItems.map(item => ({...item}));
    } else if (props.widget.targetDate) {
        // Migration from single countdown
        editItems.value = [{
            id: crypto.randomUUID(),
            label: props.widget.label || 'Countdown',
            targetDate: props.widget.targetDate
        }];
    } else {
        editItems.value = [{
            id: crypto.randomUUID(),
            label: '',
            targetDate: ''
        }];
    }
};

interface TimeLeft {
    years: number;
    months: number;
    days: number;
    hours: number;
    minutes: number;
    seconds: number;
    isExpired: boolean;
}

const itemsTimeLeft = ref<Record<string, TimeLeft>>({});

let timer: ReturnType<typeof setInterval> | null = null;

const calculateTimeLeftForItem = (targetDate: string): TimeLeft => {
    const now = dayjs();
    const target = dayjs(targetDate);

    if (target.isBefore(now)) {
        return {years: 0, months: 0, days: 0, hours: 0, minutes: 0, seconds: 0, isExpired: true};
    }

    let temp = now;

    const years = target.diff(temp, 'year');
    temp = temp.add(years, 'year');

    const months = target.diff(temp, 'month');
    temp = temp.add(months, 'month');

    const days = target.diff(temp, 'day');
    temp = temp.add(days, 'day');

    const hours = target.diff(temp, 'hour');
    temp = temp.add(hours, 'hour');

    const minutes = target.diff(temp, 'minute');
    temp = temp.add(minutes, 'minute');

    const seconds = target.diff(temp, 'second');

    return {
        years,
        months,
        days,
        hours,
        minutes,
        seconds,
        isExpired: false
    };
};

const calculateAllTimeLeft = () => {
    if (!props.widget.isConfigured) return;

    const newTimeLeft: Record<string, TimeLeft> = {};

    if (props.widget.countdownItems) {
        props.widget.countdownItems.forEach(item => {
            newTimeLeft[item.id] = calculateTimeLeftForItem(item.targetDate);
        });
    } else if (props.widget.targetDate) {
        // Legacy support
        newTimeLeft['legacy'] = calculateTimeLeftForItem(props.widget.targetDate);
    }

    itemsTimeLeft.value = newTimeLeft;
};

const addItem = () => {
    if (editItems.value.length < 2) {
        editItems.value.push({
            id: crypto.randomUUID(),
            label: '',
            targetDate: ''
        });
    }
};

const removeItem = (index: number) => {
    editItems.value.splice(index, 1);
};

const save = () => {
    const validItems = editItems.value.filter(item => item.targetDate);
    if (validItems.length === 0) return;

    emit('update', {
        ...props.widget,
        label: editWidgetTitle.value,
        countdownItems: validItems,
        isConfigured: true,
        targetDate: undefined // Clear legacy field
    });
};

const enterEditMode = () => {
    initEditMode();
    emit('update', {
        ...props.widget,
        isConfigured: false
    });
};

const startTimer = () => {
    if (timer) clearInterval(timer);
    if (props.widget.isConfigured) {
        calculateAllTimeLeft();
        timer = setInterval(calculateAllTimeLeft, 1000);
    }
};

watch(() => props.widget.isConfigured, (newVal) => {
    if (newVal) {
        startTimer();
    } else {
        if (timer) clearInterval(timer);
    }
});

watch(() => props.widget.countdownItems, () => {
    if (props.widget.isConfigured) {
        startTimer();
    }
}, {deep: true});

watch(() => props.widget.targetDate, () => {
    if (props.widget.isConfigured) {
        startTimer();
    }
});

onMounted(() => {
    startTimer();
});

onBeforeUnmount(() => {
    if (timer) clearInterval(timer);
});

// const formattedValue = computed(() => {
//     if (!props.widget.targetDate) return 'Select Date & Time';
//     return dayjs(props.widget.targetDate).format('YYYY-MM-DD HH:mm:ss');
// });
//
// const valueUpdated = (e) => {
//     props.widget.targetDate = dayjs(e).format('YYYY-MM-DD HH:mm:ss');
// }
//
// const timestamp = ref("Select Date & Time");
// const formatted = computed(() => timestamp.value ? dayjs(timestamp.value).format('YYYY-MM-DD HH:mm:ss') : '');

</script>

<template>
    <div class="card base cx-h-100 d-flex-col">
        <div class="card-header d-flex justify-between align-center">
      <span>
        <i class="ti ti-hourglass-high"></i>
        {{ widget.isConfigured ? (widget.label || 'Countdown') : 'Configure Countdown' }}
      </span>
            <div class="dropdown" data-cx-dropdown>
                <button class="cx-button small p-0-important dropdown-toggle no-caret" data-cx-toggle="dropdown">
                    <i class="ti ti-dots-vertical"></i>
                </button>
                <ul class="dropdown-menu dropdown-menu-end">
                    <li><a class="dropdown-item" href="#" @click.prevent="enterEditMode"><i
                        class="ti ti-edit"></i> Edit</a></li>
                    <li><a class="dropdown-item text-danger" href="#" @click.prevent="$emit('remove', widget.id)"
                           data-bs-dismiss="dropdown"><i class="ti ti-trash"></i> Remove</a></li>
                </ul>
            </div>
        </div>

        <div class="card-body d-flex-col justify-center align-center text-center flex-grow">
            <!-- Edit Mode -->
            <div v-if="!widget.isConfigured" class="cx-w-100">
                <div class="mb-3">
                    <label class="form-label d-block text-start">Widget Title</label>
                    <input v-model="editWidgetTitle" type="text" class="form-control form-control-sm"
                           placeholder="Title for the whole widget">
                </div>

                <div v-for="(item, index) in editItems" :key="item.id"
                     class="countdown-edit-item p-2 mb-2 border">
                    <div class="d-flex justify-between align-center mb-1">
                        <span class="small fw-bold">Countdown #{{ index + 1 }}</span>
                        <button v-if="editItems.length > 1" class="cx-button small text-danger p-0"
                                @click="removeItem(index)">
                            <i class="ti ti-x"></i>
                        </button>
                    </div>
                    <div class="mb-2">
                        <input v-model="item.label" type="text" class="form-control form-control-xs"
                               placeholder="Label (e.g. Vacation)">
                    </div>
                    <div class="mb-1">
                        <div class="dropdown" data-cx-dropdown>
                            <div data-cx-toggle="dropdown">
                                <span class="input-group-text"><i class="ti ti-calendar"></i></span>
                                <input
                                    type="text"
                                    class="cx-input"
                                    :value="item.targetDate ? dayjs(item.targetDate).format('YYYY-MM-DD HH:mm:ss') : ''"
                                    readonly
                                    placeholder="Select Date & Time"
                                >
                            </div>

                            <div class="dropdown-menu dropdown-menu-center" style="z-index: 9999">
                                <DateTimePicker v-model="item.targetDate"/>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="cx-button-group">
                    <button v-if="editItems.length < 2" class="cx-button outlined small" @click="addItem">
                        <i class="ti ti-plus"></i> Add Countdown
                    </button>

                    <button class="cx-button small" @click="save">Save Widget</button>
                </div>
            </div>

            <!-- Display Mode -->
            <div v-else class="cx-w-100 d-flex-col g-3 py-2">
                <!-- Handle multiple items -->
                <template v-if="widget.countdownItems && widget.countdownItems.length > 0">
                    <div v-for="item in widget.countdownItems" :key="item.id" class="countdown-item-display">
                        <div class="item-label small text-muted mb-1">{{ item.label || 'Countdown' }}</div>

                        <div v-if="itemsTimeLeft[item.id]?.isExpired" class="expired-msg py-2">
                            <h5 class="mb-0 text-danger">Time's up!</h5>
                        </div>

                        <div v-else class="countdown-display d-flex justify-center g-1">
                            <div v-if="itemsTimeLeft[item.id]?.years > 0" class="time-unit">
                                <div class="value">{{ itemsTimeLeft[item.id]?.years }}</div>
                                <div class="label">Y</div>
                            </div>
                            <div v-if="itemsTimeLeft[item.id]?.months > 0" class="time-unit">
                                <div class="value">{{ itemsTimeLeft[item.id]?.months }}</div>
                                <div class="label">M</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft[item.id]?.days }}</div>
                                <div class="label">D</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft[item.id]?.hours }}</div>
                                <div class="label">H</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft[item.id]?.minutes }}</div>
                                <div class="label">M</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft[item.id]?.seconds }}</div>
                                <div class="label">S</div>
                            </div>
                        </div>
                    </div>
                </template>

                <!-- Handle legacy single item -->
                <template v-else-if="widget.targetDate">
                    <div class="countdown-item-display">
                        <div v-if="itemsTimeLeft['legacy']?.isExpired" class="expired-msg">
                            <h3 class="mb-0">Time's up!</h3>
                            <p class="text-muted small">{{ widget.label }}</p>
                        </div>

                        <div v-else class="countdown-display d-flex g-2 justify-center">
                            <div v-if="itemsTimeLeft['legacy']?.years > 0" class="time-unit">
                                <div class="value">{{ itemsTimeLeft['legacy']?.years }}</div>
                                <div class="label">Yrs</div>
                            </div>
                            <div v-if="itemsTimeLeft['legacy']?.months > 0" class="time-unit">
                                <div class="value">{{ itemsTimeLeft['legacy']?.months }}</div>
                                <div class="label">Mon</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft['legacy']?.days }}</div>
                                <div class="label">Days</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft['legacy']?.hours }}</div>
                                <div class="label">Hrs</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft['legacy']?.minutes }}</div>
                                <div class="label">Min</div>
                            </div>
                            <div class="time-unit">
                                <div class="value">{{ itemsTimeLeft['legacy']?.seconds }}</div>
                                <div class="label">Sec</div>
                            </div>
                        </div>
                    </div>
                </template>
            </div>
        </div>
    </div>
</template>

<style scoped>
.countdown-display .time-unit {
    background: light-dark(var(--brand-200), var(--brand-500));
    padding: 8px;
    border-radius: 4px;
    min-width: 50px;
}

.countdown-display .time-unit.mini {
    padding: 4px;
    min-width: 35px;
}

.countdown-display .value {
    font-size: 1.2rem;
    font-weight: bold;
}

.countdown-display .time-unit.mini .value {
    font-size: 1rem;
}

.countdown-display .label {
    font-size: 0.7rem;
    text-transform: uppercase;
    opacity: 0.7;
}

.countdown-display .time-unit.mini .label {
    font-size: 0.6rem;
}

.form-control-xs {
    height: calc(1.5em + 0.5rem + 2px);
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    border-radius: 0.2rem;
}

.btn-xs {
    padding: 0.1rem 0.4rem;
    font-size: 0.7rem;
    border-radius: 0.2rem;
}

.no-caret::after {
    display: none !important;
}
</style>
