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
<style scoped>
@font-face {
    font-family: digital;
    font-style: normal;
    font-weight: 400;
    src: url("../assets/icons/webfont/digital-dismay.regular.otf") format("opentype");
}

@keyframes blink {
    50% {
        opacity: 0;
    }
}

.donkers-clock {
    /*--white-color: #fff;*/
    --black-color: #18191a;
    --red-color: #e74c3c;
}

.donkers-clock {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 60px;
    position: relative;
    width: 200px;
    height: 200px;

    .digital, .date {
        color: #389844;
    }

    .digital-back, .date-back {
        color: light-dark(rgba(0,0,0,0.05),rgba(255,255,255,0.15));
        border:1px solid var(--separator) !important;
    }

    .indicator {
        position: absolute;
        height: 10px;
        width: 10px;
        display: flex;
        justify-content: center;
        z-index: 100;

        &::after {
            content: "";
            position: absolute;
            height: 100%;
            width: 100%;
            border-radius: 50%;
            z-index: 39;
            background: light-dark(black, white);
            border: 3px solid var(--red-color);
        }

        .hand {
            position: absolute;
            height: 80px;
            width: 2px;
            bottom: 5px;
            border-radius: 25px;
            transition: none;
            transform-origin: bottom;
            background: var(--red-color);
            z-index: 30;
            /*noinspection CssUnresolvedCustomProperty*/
            box-shadow: var(--shadow-x) var(--shadow-y) 4px rgba(33, 33, 33, 0.25);
            will-change: transform;

            &.minute {
                height: 75px;
                width: 4px;
                background: var(--on-bg-surface);
                /*noinspection CssUnresolvedCustomProperty*/
                box-shadow: var(--shadow-mx) var(--shadow-my) 4px rgba(33, 33, 33, 0.25);
            }

            &.hour {
                height: 55px;
                width: 4px;
                z-index: 15;
                background: var(--on-bg-surface);
                /*noinspection CssUnresolvedCustomProperty*/
                box-shadow: var(--shadow-hx) var(--shadow-hy) 4px rgba(33, 33, 33, 0.25);
            }
        }
    }

    .clock {
        display: flex;
        height: 200px;
        width: 200px;
        border-radius: 50%;
        align-items: center;
        justify-content: center;
        /*noinspection CssInvalidFunction*/
        background: var(--bg-surface);
        border: 1px solid var(--separator);
        position: relative;
        z-index: 3;

        label {
            position: absolute;
            inset: 5px;
            text-align: center;
            /*noinspection CssUnresolvedCustomProperty*/
            transform: rotate(calc(var(--i) * (360deg / 12)));

            .h24 {
                display: inline-block;
                font-size: 7px;
                font-weight: 600;
                color:var(--brand-500);
                z-index: 2;
                /*noinspection CssUnresolvedCustomProperty*/
                transform: rotate(calc(var(--t) * (-360deg / 12)));
                will-change: transform;
            }

            span {
                display: inline-block;
                font-size: 15px;
                font-weight: 600;
                color: var(--on-bg-surface);
                /*noinspection CssUnresolvedCustomProperty*/
                transform: rotate(calc(var(--i) * (-360deg / 12)));
                will-change: transform;
            }

            &.h24l {
                position: absolute;
                inset: 18px;
                text-align: center;
                /*noinspection CssUnresolvedCustomProperty*/
                transform: rotate(calc(var(--t) * (360deg / 12)));
                will-change: transform;
            }
        }

        .minute-marker {
            position: absolute;
            width: 0;
            height: 50%;
            top: 0;
            left: 50%;
            transform-origin: center bottom;

            span {
                display: block;
                opacity: 0.25;
                background:var(--red-color);
                width: 2px;
                height: 5px;
                transform: translate(-50%, 0);
            }

            .large-marker {
                height: 8px !important;
                width: 3px;
                background:var(--on-bg-surface);
            }

            .small-marker {
                height: 5px !important;
                width: 1px !important;
                background:var(--on-bg-surface);
            }
        }
    }

    .digital {
        font-family: digital, sans-serif;
        font-size: 17px;
        position: absolute;
        top: 65.5%;
        z-index: 3;
        border-radius:5px;
        padding-left:10px;
        padding-right:10px;

        .digital-hours, .digital-minutes {
            font-family: digital, sans-serif;
            letter-spacing: 0 !important;
        }
    }

    .digital-back {
        display: inline;
        font-family: digital, sans-serif;
        font-size: 17px;
        position: absolute;
        top: 65%;
        z-index: 3;
        border:1px solid transparent;
        border-radius:5px;
        padding-left:10px;
        padding-right:10px;
        letter-spacing: 0 !important;
    }

    .date {
        font-family: "digital", sans-serif;
        font-size: 17px;
        position:absolute;
        top: 43%;
        right: 19%;
        border:1px solid transparent;
        border-radius:5px;
        z-index:4;
        width: 30px;
        text-align: center;
    }

    .date-back {
        font-family: "digital", sans-serif;
        font-size: 17px;
        position:absolute;
        top: 43%;
        right: 19%;
        border:1px solid var(--separator);
        border-radius:5px;
        z-index:3;
        width: 30px;
        text-align: center;
    }

    .colon {
        display: inline-block;
        font-family: digital, sans-serif;
        width: 7px;
        animation: blink 1s step-start infinite;
    }
}
</style>
