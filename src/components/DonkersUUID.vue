<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import CopyToClipboard from "@/components/CopyToClipboard.vue";

type UUIDVersion = 'v1' | 'v4' | 'v7' | 'nil';

const props = defineProps<{
    defaultVersion: UUIDVersion;
}>();

const activeVersion = ref<UUIDVersion>(props.defaultVersion);
const bulkCount = ref(12);
const generatedUUIDs = ref<string[]>([""]);
const singleUUID = ref("");

const handleUuidChange = async () => {
    singleUUID.value = await invoke('generate_uuid', { version: activeVersion.value });
};

const bulkGenerate = async () => {
    if (activeVersion.value === 'nil') {
        generatedUUIDs.value = [];
        return;
    }
    generatedUUIDs.value = await invoke('generate_bulk_uuid', { 
        version: activeVersion.value, 
        count: bulkCount.value 
    });
};

watch(activeVersion, () => {
    handleUuidChange();
    bulkGenerate();
});

onMounted(async () => {
    await handleUuidChange();
    generatedUUIDs.value = await invoke('generate_bulk_uuid', { 
        version: activeVersion.value, 
        count: 12 
    });
});
</script>

<template>
    <div :class="`donkers-app-uuid`">
        <div class="uuid-app-options">
          <div class="form-check form-check-inline">
            <input class="form-check-input" type="radio" id="uuid_v1" v-model="activeVersion" name="uuid_type" value="v1">
            <label class="form-check-label" for="uuid_v1">UUID v1</label>
          </div>
          <div class="form-check form-check-inline">
            <input class="form-check-input" type="radio" id="uuid_v4" v-model="activeVersion" name="uuid_type" value="v4">
            <label class="form-check-label" for="uuid_v4">UUID v4</label>
          </div>
          <div class="form-check form-check-inline">
            <input class="form-check-input" type="radio" id="uuid_v7" v-model="activeVersion" name="uuid_type" value="v7">
            <label class="form-check-label" for="uuid_v7">UUID v7</label>
          </div>
          <div class="form-check form-check-inline">
            <input class="form-check-input" type="radio" id="uuid_nil" v-model="activeVersion" name="uuid_type" value="nil">
            <label class="form-check-label" for="uuid_nil">Nil/Empty</label>
          </div>
        </div>
        <div class="uuid-single-option">
            <div class="option-title">Your <span id="uuid_version">Version {{ activeVersion }}</span> UUID</div>
            <div class="input-wrapper">
              <input class="me-2" id="single_uuid" :value="singleUUID" />
                <div class="btn-group" role="group" aria-label="Option Buttons">
                  <CopyToClipboard :text="singleUUID" size="md" color="secondary" />
                  <button v-if="activeVersion!=='nil'" @click="handleUuidChange()" class="btn btn-secondary">
                      <i class="ti ti-refresh"></i>
                  </button>
                </div>
            </div>
        </div>

        <div style="min-width:1000px;max-width:1000px;align-self: center" class="card mt-3">
            <div class="card-header">Bulk generate UUID {{ activeVersion }}</div>
            <div class="card-body">
                <div class="d-flex flex-row" style="width:100%;height:100%">
                    <div class="d-flex flex-column pr-2" style="width:50%;border-right:1px dashed var(--bs-border-color)">
                        <div class="d-flex flex-row gap-2 align-items-center">
                            <div>How many?</div>
                            <div>
                              <input class="form-control" type="number" min="0" max="100" step="1" v-model="bulkCount" :disabled="activeVersion==='nil'" />
                            </div>
                            <div><button class="btn btn-primary" @click="bulkGenerate" :disabled="activeVersion==='nil'">Generate</button></div>
                        </div>
                        <div class="mt-2">
                            <textarea style="font-family:monospace" spellcheck="false" rows="15" cols="50" :readonly="true" class="noresize" :disabled="activeVersion==='nil'">{{ generatedUUIDs.join('\n') }}</textarea>
                        </div>
                    </div>
                    <div class="d-flex flex-column ps-2" style="width:50%">
                        <div><strong>About</strong></div>
                        <div>
                            <span v-if="activeVersion==='v1'">A Version 1 UUID is a universally unique identifier that is generated using a timestamp and the MAC address of the computer on which it was generated.</span>
                            <span v-if="activeVersion==='v4'">A Version 4 UUID is a universally unique identifier that is generated using random numbers. The Version 4 UUIDs produced by this tool were generated using a secure random number generator.</span>
                            <span v-if="activeVersion==='v7'">A Version 7 UUID is a universally unique identifier that is generated using a timestamp, a counter and a cryptographically strong random number. Generally, Version 7 UUIDs have better entropy (i.e. randomness) than Version 1 UUIDs.</span>
                            <span v-if="activeVersion==='nil'">A Nil/Empty UUID is a special form that contains all zeros.</span>
                        </div>
                        <div class="mt-2" style="color:#aaa">
                            The UUIDs generated by this tool are provided AS IS without warranty of any kind, not even the warranty that the generated UUIDs are actually unique. You are responsible for using the UUIDs and assume any risk inherent to using them. You are not permitted to use the UUIDs generated by this application if you do not agree to these terms.
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
