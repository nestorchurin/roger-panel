import { create } from 'zustand';
import { authApi, serversApi } from '../api/client';
import type { User, Server } from '../api/client';

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (username: string, email: string, password: string) => Promise<void>;
  logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  token: localStorage.getItem('token'),
  isAuthenticated: !!localStorage.getItem('token'),

  login: async (email, password) => {
    const { data } = await authApi.login({ email, password });
    localStorage.setItem('token', data.token);
    set({ user: data.user, token: data.token, isAuthenticated: true });
  },

  register: async (username, email, password) => {
    const { data } = await authApi.register({ username, email, password });
    localStorage.setItem('token', data.token);
    set({ user: data.user, token: data.token, isAuthenticated: true });
  },

  logout: () => {
    localStorage.removeItem('token');
    set({ user: null, token: null, isAuthenticated: false });
  },
}));

interface ServersState {
  servers: Server[];
  loading: boolean;
  fetchServers: () => Promise<void>;
  createServer: (data: Partial<Server>) => Promise<void>;
}

export const useServersStore = create<ServersState>((set) => ({
  servers: [],
  loading: false,

  fetchServers: async () => {
    set({ loading: true });
    const { data } = await serversApi.list();
    set({ servers: data, loading: false });
  },

  createServer: async (data) => {
    const { data: server } = await serversApi.create(data);
    set((state) => ({ servers: [server, ...state.servers] }));
  },
}));
