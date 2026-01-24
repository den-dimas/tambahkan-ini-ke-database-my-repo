import api from './api';

export interface AuthResponse {
  token: string;
  username: string;
}

export const authService = {
  async login(payload: any) {
    const { data } = await api.post('/auth/login', payload);
    return data;
  },

  async register(payload: any) {
    const { data } = await api.post('/auth/register', payload);
    return data;
  },
};
