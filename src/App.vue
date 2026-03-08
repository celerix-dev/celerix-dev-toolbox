<script setup lang="ts">
import { RouterView } from 'vue-router';
import { onMounted } from 'vue';
import { useUserStore } from '@/stores/user';

const userStore = useUserStore();

onMounted(async () => {
  if (!userStore.isInitialized) {
    await userStore.loadUser();
  }
});

</script>

<template>
  <RouterView />
</template>
