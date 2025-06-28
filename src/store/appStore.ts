import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { AppState, Theme } from './types';

// 应用状态管理接口
interface AppStore extends AppState {
  // 状态更新方法
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setTheme: (theme: Theme) => void;
  clearError: () => void;
  
  // 工具方法
  reset: () => void;
}

// 初始状态
const initialState: AppState = {
  theme: 'system',
  loading: false,
  error: null,
};

// 创建应用状态store
export const useAppStore = create<AppStore>()(
  devtools(
    subscribeWithSelector(
      (set) => ({
      // 初始状态
      ...initialState,
      
      // 状态更新方法
      setLoading: (loading) => {
        set({ loading }, false, 'app/setLoading');
      },
      
      setError: (error) => {
        set({ error }, false, 'app/setError');
      },
      
      setTheme: (theme) => {
        set({ theme }, false, 'app/setTheme');
        // 可以在这里添加主题切换的副作用
        if (typeof window !== 'undefined') {
          localStorage.setItem('theme', theme);
        }
      },
      
      clearError: () => {
        set({ error: null }, false, 'app/clearError');
      },
      
      // 工具方法
      reset: () => {
        set(initialState, false, 'app/reset');
      },
    })
    ),
    {
      name: 'app-store',
    }
  )
);

// 初始化主题
if (typeof window !== 'undefined') {
  const savedTheme = localStorage.getItem('theme') as Theme;
  if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
    useAppStore.getState().setTheme(savedTheme);
  }
}