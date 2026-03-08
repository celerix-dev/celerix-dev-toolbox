<script setup lang="ts">
import DonkersJWT from "@/components/DonkersJWT.vue";
</script>

<template>

  <teleport to="#breadcrumbs">
    <div><i class="ti ti-password-fingerprint"></i> <strong>JWT Inspector</strong></div>
  </teleport>

  <div class="jwt-index mini-app cx-h-100 pb-3">
    <DonkersJWT />
  </div>
</template>

<style scoped>
</style>
