<script setup lang="ts">
import DonkersBase64 from "@/components/DonkersBase64.vue";
</script>

<template>
  <teleport to="#breadcrumbs">
    <div><i class="ti ti-file-code-2"></i> <strong>Base64 En/Decoder</strong></div>
  </teleport>

  <div class="d-flex mini-app">
    <DonkersBase64 />
  </div>
</template>
