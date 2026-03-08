<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import CopyToClipboard from "@/components/CopyToClipboard.vue";

interface JwtParts {
  header: string;
  payload: string;
}

const token = ref('');
const decodedHeader = ref('');
const decodedPayload = ref('');
const isError = ref(false);
const errorMessage = ref('');

const highlightRef = ref<HTMLElement | null>(null);
const textareaRef = ref<HTMLTextAreaElement | null>(null);

const handleScroll = () => {
  if (textareaRef.value && highlightRef.value) {
    highlightRef.value.scrollTop = textareaRef.value.scrollTop;
  }
};

const processToken = async () => {
  if (!token.value) {
    decodedHeader.value = '';
    decodedPayload.value = '';
    isError.value = false;
    errorMessage.value = '';
    return;
  }

  try {
    const result: JwtParts = await invoke('decode_jwt', { token: token.value });

    // Format JSON
    try {
        decodedHeader.value = JSON.stringify(JSON.parse(result.header), null, 2);
    } catch {
        decodedHeader.value = result.header;
    }

    try {
        decodedPayload.value = JSON.stringify(JSON.parse(result.payload), null, 2);
    } catch {
        decodedPayload.value = result.payload;
    }

    isError.value = false;
    errorMessage.value = '';
  } catch (e) {
    isError.value = true;
    errorMessage.value = String(e);
    decodedHeader.value = '';
    decodedPayload.value = '';
  }
};

watch(token, () => {
  processToken();
});

const escapeHtml = (unsafe: string) => {
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
 };

const highlightedToken = computed(() => {
  const parts = token.value.split('.');
  let html = '';

  if (parts[0]) {
    html += `<span class="jwt-header">${escapeHtml(parts[0])}</span>`;
  }
  if (parts.length > 1) {
    html += '.';
    if (parts[1]) {
      html += `<span class="jwt-payload">${escapeHtml(parts[1])}</span>`;
    }
  }
  if (parts.length > 2) {
    html += '.';
    if (parts[2]) {
      html += `<span class="jwt-signature">${escapeHtml(parts[2])}</span>`;
    }
  }

  // If there are more parts (invalid but still)
  if (parts.length > 3) {
      for (let i = 3; i < parts.length; i++) {
          html += '.' + escapeHtml(parts[i]);
      }
  }

  // Add a trailing newline if needed to match textarea behavior for scrolling
  if (token.value.endsWith('\n')) {
      html += '\n';
  }

  return html;
});

const openLink = async (url: string) => {
  await openUrl(url);
};

</script>

<template>
  <div class="donkers-app-jwt cx-h-100">
    <div class="d-flex-row cx-h-100 g-3">
      <div class="d-flex-col cx-h-100">

        <div class="alert alert-secondary" role="alert">
          <h5 class="alert-heading">JSON Web Token</h5>
          Decode, verify, and generate JSON Web Tokens, which are an open, industry standard RFC 7519 method for representing claims securely between two parties. Pasting a jwt token below will decode the parts safely.<br/>
          <a href="#" @click.prevent="openLink('https://en.wikipedia.org/wiki/JSON_Web_Token')">Learn more about JWT</a>&nbsp;
          <a href="#" @click.prevent="openLink('https://jwt.io/libraries')">See JWT libraries</a>
        </div>

        <div class="card cx-h-100 d-flex-col">
          <div class="card-header d-flex justify-between">
            Encoded
            <div class="d-flex gap-2 align-center">
              <CopyToClipboard :text="token" size="sm" />
              <button class="cx-button small" @click="token=''">
                <i class="ti ti-refresh"></i>
              </button>
            </div>
          </div>
          <div class="card-body flex-grow p-0">
            <div class="jwt-editor-wrapper cx-h-100">
              <div
                ref="highlightRef"
                class="jwt-editor-highlight"
                v-html="highlightedToken"
              ></div>
              <textarea
                ref="textareaRef"
                v-model="token"
                class="jwt-editor-textarea"
                spellcheck="false"
                placeholder="Paste your JWT here..."
                @scroll="handleScroll"
              ></textarea>
            </div>
          </div>
        </div>
      </div>

      <div class="d-flex-col cx-h-100">

        <div class="alert alert-warning" role="alert">
          <h5 class="alert-heading">Notice</h5>
          For data protection, all JWT debugging and validation happens in the application. Be aware where you paste or share JWTs as they can represent credentials that grant access to resources. This application does not store or transmit your JSON Web Tokens outside of the application.
        </div>

        <div class="card cx-h-100 d-flex-col">
          <div class="card-header">Decoded</div>
          <div class="card-body flex-grow overflow-auto">
            <div v-if="isError" class="p-3 text-danger">
              {{ errorMessage }}
            </div>
            <template v-else-if="token">
              <div class="decoded-section">
                <div class="section-label jwt-header">Header <CopyToClipboard :text="decodedHeader" color="secondary" /></div>
                <pre><code>{{ decodedHeader }}</code></pre>
              </div>
              <div class="decoded-section">
                <div class="section-label jwt-payload">Payload <CopyToClipboard :text="decodedPayload" color="secondary" /></div>
                <pre><code>{{ decodedPayload }}</code></pre>
              </div>
            </template>
            <div v-else class="text-muted">
              Paste a token to see its decoded content.
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.donkers-app-jwt {
    width: 100%;
    height: 100%;
}

.donkers-app-jwt .jwt-editor-wrapper {
    position: relative;
    border: 1px solid var(--separator);
    border-radius: 4px;
    background: var(--bg-surface);
    overflow: hidden;
}

.donkers-app-jwt .jwt-editor-textarea,
.donkers-app-jwt .jwt-editor-highlight {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    padding: 15px;
    margin: 0;
    border: none;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'source-code-pro', monospace;
    line-height: 1.5;
    white-space: pre-wrap;
    word-wrap: break-word;
    overflow-y: auto;
}

.donkers-app-jwt .jwt-editor-textarea {
    background: transparent;
    color: transparent;
    caret-color: var(--bg-base);
    resize: none;
    z-index: 2;
    outline: none;
}

.donkers-app-jwt .jwt-editor-highlight {
    z-index: 1;
    color: var(--bg-base);
    pointer-events: none;
}

.donkers-app-jwt .jwt-header { color: #fb015b; }
.donkers-app-jwt .jwt-payload { color: #d63aff; }
.donkers-app-jwt .jwt-signature { color: #00b9f1; }

.donkers-app-jwt .decoded-section {
    padding-bottom: 15px;
    margin-bottom: 15px;
    border-bottom: 1px solid var(--separator);
}

.donkers-app-jwt .decoded-section:last-child {
    border-bottom: none;
}

.donkers-app-jwt pre {
    margin: 0;
    color: var(--bg-base);
    white-space: pre-wrap;
    word-wrap: break-word;
}

.donkers-app-jwt .section-label {
    font-weight: bold;
    margin-bottom: 10px;
    text-transform: uppercase;
    color: var(--brand);
}

</style>
