<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import { EditorState } from '@codemirror/state';
import { EditorView, basicSetup } from 'codemirror';
import { json } from '@codemirror/lang-json';
import { oneDark } from '@codemirror/theme-one-dark';

const props = defineProps<{
  content: string;
  readOnly?: boolean;
}>();

const editorContainer = ref<HTMLElement | null>(null);
let view: EditorView | null = null;

const getTheme = () => {
  return document.documentElement.getAttribute('data-bs-theme') || 'light';
};

const createEditor = () => {
  if (!editorContainer.value) return;

  const isDark = getTheme() === 'dark';
  const extensions = [
    basicSetup,
    json(),
    EditorState.readOnly.of(props.readOnly !== false),
    EditorView.editable.of(props.readOnly === false),
  ];

  if (isDark) {
    extensions.push(oneDark);
  }

  const state = EditorState.create({
    doc: props.content,
    extensions
  });

  if (view) {
    view.destroy();
  }

  view = new EditorView({
    state,
    parent: editorContainer.value
  });
};

onMounted(() => {
  createEditor();

  // Watch for theme changes
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === 'data-bs-theme') {
        createEditor();
      }
    });
  });

  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-bs-theme']
  });

  onBeforeUnmount(() => {
    observer.disconnect();
    if (view) {
      view.destroy();
    }
  });
});

watch(() => props.content, (newContent) => {
  if (view && newContent !== view.state.doc.toString()) {
    view.dispatch({
      changes: { from: 0, to: view.state.doc.length, insert: newContent }
    });
  }
});
</script>

<template>
  <div ref="editorContainer" class="code-editor"></div>
</template>

<style scoped>
.code-editor {
  width: 100%;
  height: 100%;
}
:deep(.cm-editor) {
  height: 100%;
  max-height: 60vh;
}
:deep(.cm-scroller) {
  overflow: auto;
}
</style>
