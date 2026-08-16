import axios from 'axios';

const api = axios.create({
  baseURL: '/api',
});

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export interface User {
  id: string;
  username: string;
  email: string;
  role: string;
  created_at: string;
  updated_at: string;
}

export interface Server {
  id: string;
  name: string;
  server_type: string;
  version: string;
  port: number;
  max_ram_mb: number;
  min_ram_mb: number;
  cpu_limit: number | null;
  iops_limit: number | null;
  net_rx_limit: number | null;
  net_tx_limit: number | null;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export const authApi = {
  register: (data: { username: string; email: string; password: string }) =>
    api.post<AuthResponse>('/auth/register', data),
  login: (data: { email: string; password: string }) =>
    api.post<AuthResponse>('/auth/login', data),
};

export const serversApi = {
  list: () => api.get<Server[]>('/servers'),
  create: (data: Partial<Server>) => api.post<Server>('/servers', data),
};

export default api;
