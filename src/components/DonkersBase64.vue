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
      <div class="w-75 h-100" style="">
        <div class="card mb-2">
          <div class="card-header d-flex justify-content-between">Input <button class="btn btn-primary btn-sm" @click="process">
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
          <div class="card-header d-flex justify-content-between">Result <CopyToClipboard :text="outputText" /></div>
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
