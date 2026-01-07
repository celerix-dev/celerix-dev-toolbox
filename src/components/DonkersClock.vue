<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, type CSSProperties } from "vue";
import dayjs from "dayjs";

interface Props {
    id: number | string;
    showDate?: boolean;
    showDigital?: boolean;
    show24h?: boolean;
    style?: CSSProperties;
}

const props = withDefaults(defineProps<Props>(), {
    showDate: false,
    showDigital: false,
    show24h: false,
    style: () => ({}),
});

const animationFrameId = ref<number | null>(null);

const secToDeg = ref(0);
const minToDeg = ref(0);
const hrToDeg = ref(0);
const digitalHours = ref("12");
const digitalMinutes = ref("45");
const clockDate = ref("19");

const shadowVars = ref<CSSProperties>({});

const updateShadows = () => {
    const now = new Date();
    const seconds = now.getSeconds();
    const minutes = now.getMinutes();
    const hours = now.getHours();

    const secondAngle = (seconds / 60) * 360;
    const minuteAngle = (minutes / 60) * 360 + (seconds / 60) * 6;
    const hourAngle = (hours / 12) * 360 + (minutes / 60) * 30;
    const lightAngle = 45;

    const shadowOffsetX = (angle: number) => Math.cos((angle + lightAngle) * (Math.PI / 180)) * 2;
    const shadowOffsetY = (angle: number) => Math.sin((angle + lightAngle) * (Math.PI / 180)) * 2;

    shadowVars.value = {
        "--shadow-x": `${shadowOffsetX(secondAngle)}px`,
        "--shadow-y": `${shadowOffsetY(secondAngle)}px`,
        "--shadow-mx": `${shadowOffsetX(minuteAngle)}px`,
        "--shadow-my": `${shadowOffsetY(minuteAngle)}px`,
        "--shadow-hx": `${shadowOffsetX(hourAngle)}px`,
        "--shadow-hy": `${shadowOffsetY(hourAngle)}px`,
    } as CSSProperties;
};

const updateTime = () => {
    const date = new Date();
    const milliseconds = date.getMilliseconds();
    const seconds = date.getSeconds() + milliseconds / 1000;
    const minutes = date.getMinutes() + seconds / 60;
    const hours = date.getHours() + minutes / 60;

    secToDeg.value = (seconds / 60) * 360;
    minToDeg.value = (minutes / 60) * 360;
    hrToDeg.value = (hours / 12) * 360;

    if (props.showDigital) {
        digitalHours.value = dayjs().format("HH");
        digitalMinutes.value = dayjs().format("mm");
    }

    if (props.showDate) {
        clockDate.value = dayjs().format("DD");
    }

    // if (window.$r.colorScheme.getScheme() === 'light') {
    updateShadows();
    // } else {
    // shadowVars.value = {};
    // }
    animationFrameId.value = requestAnimationFrame(updateTime);
};

onMounted(() => {
    animationFrameId.value = requestAnimationFrame(updateTime);
});

onBeforeUnmount(() => {
    if (animationFrameId.value) {
        cancelAnimationFrame(animationFrameId.value);
    }
});

const createLabelStyle = (index: number) => ({ "--i": index + 1 } as CSSProperties);
const createLabelStyle2 = (index: number) => ({ "--t": index + 1 } as CSSProperties);
</script>

<template>
    <div class="donkers-clock" :style="[style, shadowVars]">
        <div class="clock">
            <!-- 24-hour labels -->
            <template v-if="show24h">
                <label
                    v-for="index in 12"
                    :key="`24h-label-${index}`"
                    class="h24l"
                    :style="createLabelStyle2(index - 1)"
                >
                    <span class="h24">{{ index + 12 }}</span>
                </label>
            </template>

            <!-- 12-hour labels -->
            <label
                v-for="index in 12"
                :key="`12h-label-${index}`"
                :style="createLabelStyle(index - 1)"
            >
                <span>{{ index }}</span>
            </label>

            <!-- Minute markers -->
            <div
                v-for="index in 60"
                :key="`minute-marker-${index}`"
                class="minute-marker"
                :style="{ transform: `rotate(${(index - 1) * 6}deg)` }"
            >
                <span :class="index % 5 === 1 ? 'large-marker' : 'small-marker'"></span>
            </div>

            <div class="indicator">
                <span
                    class="hand hour"
                    :class="`hour-${id}`"
                    :style="{ transform: `rotate(${hrToDeg}deg)` }"
                ></span>
                <span
                    class="hand minute"
                    :class="`minute-${id}`"
                    :style="{ transform: `rotate(${minToDeg}deg)` }"
                ></span>
                <span
                    class="hand second"
                    :class="`second-${id}`"
                    :style="{ transform: `rotate(${secToDeg}deg)` }"
                ></span>
            </div>

            <!-- Digital clock -->
            <template v-if="showDigital">
                <div class="digital-back">88:88</div>
                <div class="digital" id="clock-digital">
                    <span class="digital-hours">{{ digitalHours }}</span>
                    <span class="colon">:</span>
                    <span class="digital-minutes">{{ digitalMinutes }}</span>
                </div>
            </template>

            <!-- Date display -->
            <template v-if="showDate">
                <div class="date-back">88</div>
                <div class="date" id="clock-date">{{ clockDate }}</div>
            </template>
        </div>
    </div>
</template>
