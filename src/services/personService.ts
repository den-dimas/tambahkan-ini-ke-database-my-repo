import api from './api';

export const personService = {
  async getFeed() {
    const { data } = await api.get('/people/feed');
    return data;
  },

  async getStats() {
    const { data } = await api.get('/people/stats');
    return data;
  },

  async search(query: string) {
    const { data } = await api.get('/people/search', { params: { q: query } });
    return data;
  },

  async getPerson(id: string) {
    const { data } = await api.get(`/people/${id}`);
    return data;
  },

  async createPerson(payload: any) {
    const { data } = await api.post('/people', payload);
    return data;
  },

  async listPeople(params?: any) {
    const { data } = await api.get('/people', { params });
    return data;
  },

  async proposeEdit(id: string, payload: any) {
    const { data } = await api.post(`/people/${id}/edit`, payload);
    return data;
  },
};
