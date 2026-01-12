<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useUserStore } from '@/stores/user';

const userStore = useUserStore();
const nicknameInput = ref('');
const isSaving = ref(false);
const showSuccess = ref(false);

onMounted(async () => {
  if (!userStore.isInitialized) {
    await userStore.loadUser();
  }
  nicknameInput.value = userStore.nickname;
});

const saveProfile = async () => {
  isSaving.value = true;
  showSuccess.value = false;
  try {
    await userStore.setNickname(nicknameInput.value);
    showSuccess.value = true;
    setTimeout(() => {
      showSuccess.value = false;
    }, 3000);
  } catch (e) {
    console.error('Failed to save profile', e);
  } finally {
    isSaving.value = false;
  }
};
</script>

<template>
  <teleport to="#breadcrumbs">
    <div class="d-flex align-items-center gap-2">
      <i class="ti ti-user-star"></i>
      <strong>User Profile</strong>
    </div>
  </teleport>

  <div class="container py-5">
    <div class="row justify-content-center">
      <div class="col-md-6">
        <div class="card shadow-sm border-0">
          <div class="card-body p-4">
            <div class="text-center mb-4">
              <div class="avatar-placeholder rounded-circle bg-primary text-white d-flex align-items-center justify-content-center mx-auto mb-3" style="width: 80px; height: 80px;">
                <span class="fs-1">{{ (userStore.userDisplayName[0] || 'C').toUpperCase() }}</span>
              </div>
              <h4>{{ userStore.userDisplayName }}</h4>
              <p class="text-muted small">Local Persona</p>
            </div>

            <form @submit.prevent="saveProfile">
              <div class="mb-4">
                <label class="form-label">How should we call you?</label>
                <div class="input-group">
                  <span class="input-group-text bg-transparent"><i class="ti ti-user-edit"></i></span>
                  <input 
                    v-model="nicknameInput" 
                    type="text" 
                    class="form-control shadow-none" 
                    placeholder="Enter your nickname"
                    maxlength="30"
                  >
                </div>
                <div class="form-text mt-2">
                  This name is stored locally on this machine and used across the application.
                </div>
              </div>

              <div class="d-grid gap-2">
                <button type="submit" class="btn btn-primary" :disabled="isSaving || !nicknameInput.trim()">
                  <span v-if="isSaving" class="spinner-border spinner-border-sm me-1"></span>
                  <i v-else class="ti ti-device-floppy me-1"></i>
                  Save Changes
                </button>
              </div>

              <div v-if="showSuccess" class="alert alert-success mt-3 py-2 text-center" role="alert">
                <i class="ti ti-check me-1"></i> Profile updated successfully!
              </div>
            </form>
          </div>
        </div>

        <div class="card mt-4 shadow-sm border-0 bg-secondary-subtle">
          <div class="card-body">
            <h6 class="card-title"><i class="ti ti-info-circle me-1 text-primary"></i> Future Synchronization</h6>
            <p class="card-text small text-muted">
              Currently, your profile is local-only. In future updates, we plan to introduce synchronization features that will allow you to link this persona across multiple devices.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.avatar-placeholder {
  user-select: none;
}
</style>
