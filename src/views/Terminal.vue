<script setup lang="ts">
import CelerixTerminal from "@/components/CelerixTerminal.vue";
import {nextTick, onBeforeUnmount, onMounted, ref, watch} from "vue";
import {getCurrentWindow} from "@tauri-apps/api/window";

const tabs = ref<{ id: string; name: string; isEditing: boolean }[]>([
  {
    id: Math.random().toString(36).substring(7),
    name: 'Terminal',
    isEditing: false,
  }
]);
const activeTabId = ref(tabs.value[0].id);
const terminalRefs = ref<Record<string, any>>({});

onMounted(() => {
  // but for a standalone window, just removing it is fine.
  document.body.classList.remove('ps-md-sbwidth');
});

onBeforeUnmount(() => {
  // If this window is ever reused for the dashboard, add it back
  document.body.classList.add('ps-md-sbwidth');
});

const startRename = (tab: any) => {
  tab.isEditing = true;
  // Use nextTick to focus the input automatically
  nextTick(() => {
    const input = document.getElementById(`input-${tab.id}`);
    input?.focus();
  });
};

const saveRename = (tab: any) => {
  tab.isEditing = false;
  if (!tab.name.trim()) tab.name = 'Terminal';
};

const addTab = async () => {
  const newId = Math.random().toString(36).substring(7);
  tabs.value.push({ id: newId, name: 'Terminal', isEditing: false });
  activeTabId.value = newId;
};

const removeTab = (id: string) => {
  const index = tabs.value.findIndex(t => t.id === id);
  if (index !== -1) {
    tabs.value.splice(index, 1);
    delete terminalRefs.value[id];
    if (tabs.value.length === 0) {
      getCurrentWindow().close();
    } else if (activeTabId.value === id) {
      activeTabId.value = tabs.value[Math.max(0, index - 1)].id;
    }
  }
};

watch(activeTabId, async (newId) => {
  await nextTick();
  if (terminalRefs.value[newId]) {
    terminalRefs.value[newId].fit();
    terminalRefs.value[newId].focus();
  }
});

</script>

<template>
  <div class="w-100 bg-transparent d-flex flex-column" style="overflow: hidden; height: 100vh;">
    <div class="terminal-tabs d-flex align-items-center bg-secondary-subtle">
      <div
          v-for="tab in tabs"
          :key="tab.id"
          class="terminal-tab px-3 py-1 border-end cursor-pointer d-flex align-items-center"
          :class="{ 'active bg-body': activeTabId === tab.id }"
          @click="activeTabId = tab.id"
          @dblclick="startRename(tab)"
      >
        <input
            v-if="tab.isEditing"
            :id="`input-${tab.id}`"
            v-model="tab.name"
            class="form-control form-control-sm py-0 px-1 border-0 bg-transparent text-body"
            style="width: 80px; font-size: 0.85rem;"
            @blur="saveRename(tab)"
            @keyup.enter="saveRename(tab)"
        />

        <span v-else class="me-2 small text-truncate" style="max-width: 100px;">
          {{ tab.name}}
        </span>
        <i class="ti ti-x small-icon" @click.stop="removeTab(tab.id)"></i>
      </div>
      <div class="px-2 cursor-pointer text-secondary" @click="addTab">
        <i class="ti ti-plus"></i>
      </div>
    </div>
    <div class="flex-grow-1 overflow-hidden">
      <div
          v-for="tab in tabs"
          :key="tab.id"
          v-show="activeTabId === tab.id"
          class="h-100"
      >
        <CelerixTerminal
            :ref="el => { if (el) terminalRefs[tab.id] = el }"
            :show-header="false"
            @exit="removeTab(tab.id)"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.terminal-tab {
  user-select: none;
  font-size: 0.85rem;
  border-bottom: 1px solid transparent;
  margin-bottom: -1px;
}

.terminal-tab.active {
  border-bottom-color: var(--bs-body-bg);
}

.cursor-pointer {
  cursor: pointer;
}

.small-icon {
  font-size: 0.75rem;
  opacity: 0.6;
}

.small-icon:hover {
  opacity: 1;
}

.terminal-tabs {
  height: 35px;
}
</style>