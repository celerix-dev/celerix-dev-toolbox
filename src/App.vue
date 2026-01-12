<script setup lang="ts">
import { RouterView } from 'vue-router';
import { onMounted, watch } from 'vue';
import { useUserStore } from '@/stores/user';
import { colorScheme } from '@/services/color-scheme';

const userStore = useUserStore();

onMounted(async () => {
  if (!userStore.isInitialized) {
    await userStore.loadUser();
  }
  colorScheme.applyTheme(userStore.theme);
});

watch(() => userStore.theme, (newTheme) => {
  colorScheme.applyTheme(newTheme);
});
</script>

<template>
  <RouterView />
</template>
