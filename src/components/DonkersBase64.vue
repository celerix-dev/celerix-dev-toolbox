<script setup lang="ts">
import {ref, watch} from 'vue';
import {invoke} from '@tauri-apps/api/core';
import CopyToClipboard from "@/components/CopyToClipboard.vue";

const selectedStyle = ref<'decode' | 'encode'>('encode');
const inputText = ref('');
const outputText = ref('');
const isError = ref(false);

watch(selectedStyle, () => {
  inputText.value = '';
  outputText.value = '';
  isError.value = false;
});

const process = async () => {
  if (inputText.value !== '') {
    isError.value = false;
    try {
      if (selectedStyle.value === 'decode') {
        outputText.value = await invoke('base64_decode', {input: inputText.value});
      } else {
        outputText.value = await invoke('base64_encode', {input: inputText.value});
      }
    } catch (e) {
      isError.value = true;
      outputText.value = 'Error: ' + e;
    }
  }
};
</script>

<template>
  <div :class="`donkers-app-base64`">
<!--    <div class="base64-app-header">-->
<!--      Base64 Encode/Decode-->
<!--    </div>-->
    <div class="base64-app-options">
      <div class="form-check form-check-inline">
        <input class="form-check-input" type="radio" id="encode" v-model="selectedStyle" name="base64_type"
               value="encode">
        <label class="form-check-label" for="encode">Encode</label><br>
      </div>
      <div class="form-check form-check-inline">
        <input class="form-check-input" type="radio" id="decode" v-model="selectedStyle" name="base64_type"
               value="decode">
        <label class="form-check-label" for="decode">Decode</label><br>
      </div>
    </div>
    <div class="base64-app-input">
      <div class="cx-w-75 cx-h-100" style="">
        <div class="card mb-2">
          <div class="card-header d-flex justify-between">Input <button class="cx-button small" @click="process">
            <span v-if="selectedStyle==='encode'">Base64 Encode</span>
            <span v-if="selectedStyle==='decode'">Base64 Decode</span>
          </button></div>
          <div class="card-body">
                <textarea
                    id="input-text"
                    v-model="inputText"
                    spellcheck="false"
                    class="noresize"
                    rows="12"
                    :placeholder="selectedStyle==='encode'?'Paste your text to encode':'Paste your base64 encoded string to decode'"
                ></textarea>
          </div>
        </div>

        <div class="card">
          <div class="card-header d-flex justify-between">Result <CopyToClipboard :text="outputText" /></div>
          <div class="card-body">
                    <textarea
                        id="output-text"
                        readonly
                        :value="outputText"
                        spellcheck="false"
                        class="noresize"
                        :class="{ 'error': isError }"
                        rows="12"
                        :placeholder="selectedStyle==='encode'?'Base64 encoded string will appear here':'Decoded text will appear here.'"
                    ></textarea>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
.mini-app {
    margin-top: 20px;

    input:not([type="checkbox"]):not([type="radio"]), textarea {
        border: 1px solid var(--bs-border-color);
        border-radius: var(--s-1);
        background: var(--bg-base) none repeat scroll 0 0;
        outline: currentcolor none medium;
        width: 100%;
        padding: var(--s-1);

        &::placeholder {
            color: var(--placeholder-color);
        }

        &:focus {
            border: 1px solid var(--bs-secondary-text-emphasis);
        }
    }

    input:not([type="checkbox"]):not([type="radio"]) {
        /*height: 32px;*/

        &:disabled {
            background: var(--bs-btn-disabled-color);
            cursor: not-allowed;
        }
    }

}
.donkers-app-base64 {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;

    .base64-app-header {
        display: flex;
        font-size: 2rem;
        justify-content: center;
        align-items: center;
        font-weight: 600;
        white-space: nowrap;

        i {
            margin-left: 10px;
            font-size: 2rem;
        }
    }

    .base64-app-options {
        display: flex;
        align-items: center;
        justify-content: center;
        //font-size: 1rem;
        margin-top: 1rem;
        margin-bottom: 1rem;
        white-space: nowrap;
    }

    .base64-app-input {
        display: flex;
        justify-content: center;

        textarea {
            font-family: monospace !important;

            &::placeholder {
                opacity: 0.35;
                /*font-size: 0.8rem;*/
            }

            &.error {
                border-color: #ff0000 !important;
            }
        }
    }
}
</style>
