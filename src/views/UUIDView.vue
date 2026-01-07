<script setup lang="ts">

import DonkersUUID from "@/components/DonkersUUID.vue";
</script>

<template>
  <teleport to="#breadcrumbs">
    <div><i class="ti ti-fidget-spinner"></i> <strong>UUID Generator</strong></div>
  </teleport>

  <div>
    <DonkersUUID default-version="v4" />
  </div>
</template>

<style scoped>

</style>