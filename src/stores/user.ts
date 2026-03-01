import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { storageService } from '@/services/storage';

interface UserState {
  nickname: string;
  theme: 'auto' | 'light' | 'dark';
}

export const useUserStore = defineStore('user', () => {
  const nickname = ref('');
  const theme = ref<'auto' | 'light' | 'dark'>('auto');
  const isInitialized = ref(false);

  const loadUser = async () => {
    try {
      const data = await storageService.load<UserState>('USER');
      if (data) {
        nickname.value = data.nickname || '';
        theme.value = data.theme || 'auto';
      } else {
        // Default values if none exists
        nickname.value = 'Celerix Pilot';
        theme.value = 'auto';
      }
      isInitialized.value = true;
    } catch (e) {
      console.error('Failed to load user state', e);
      nickname.value = 'Celerix Pilot';
      theme.value = 'auto';
    }
  };

  const saveUser = async () => {
    if (!isInitialized.value) return;
    try {
      await storageService.save('USER', {
        nickname: nickname.value,
        theme: theme.value
      });
    } catch (e) {
      console.error('Failed to save user state', e);
    }
  };

  const setNickname = async (name: string) => {
    nickname.value = name;
    await saveUser();
  };

  const setTheme = async (newTheme: 'auto' | 'light' | 'dark') => {
    theme.value = newTheme;
    await saveUser();
  };

  const userDisplayName = computed(() => nickname.value || 'Celerix Pilot');

  return {
    nickname,
    isInitialized,
    loadUser,
    setNickname,
    theme,
    setTheme,
    userDisplayName
  };
});
