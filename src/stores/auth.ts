import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || '');
  const username = ref(localStorage.getItem('username') || '');

  const isAuthenticated = computed(() => !!token.value);

  function setAuth(data: { token: string; username: string }) {
    token.value = data.token;
    username.value = data.username;
    localStorage.setItem('token', data.token);
    localStorage.setItem('username', data.username);
  }

  function logout() {
    token.value = '';
    username.value = '';
    localStorage.removeItem('token');
    localStorage.removeItem('username');
  }

  return {
    token,
    username,
    isAuthenticated,
    setAuth,
    logout,
  };
});
