// 统一状态管理入口

// 导出所有store
export { useAppStore } from './appStore';
export { useTaskStore } from './taskStore';
export { useActionStore } from './actionStore';

// 导出类型
export type {
  // 基础类型
  Task,
  Action,
  CreateTaskData,
  CreateActionData,
  UpdateTaskData,
  UpdateActionData,
  
  // 过滤器类型
  TaskFilters,
  ActionFilters,
  
  // 统计类型
  TaskStats,
  ActionStats,
  
  // 其他类型
  ExecutionResult,
  Theme,
  
  // 状态类型
  AppState,
  TaskState,
  ActionState,
  BaseState,
} from './types';

// 工具函数

/**
 * 获取所有store的当前状态
 */
import { useAppStore } from './appStore';
import { useTaskStore } from './taskStore';
import { useActionStore } from './actionStore';
export const getAllStores = () => {
  return {
    app: useAppStore.getState(),
    task: useTaskStore.getState(),
    action: useActionStore.getState(),
  };
};

/**
 * 重置所有store到初始状态
 */
export const resetAllStores = () => {
  useAppStore.getState().reset();
  useTaskStore.getState().reset();
  useActionStore.getState().reset();
};

/**
 * 状态持久化到localStorage
 */
export const persistStoreState = () => {
  if (typeof window === 'undefined') return;
  try {
    const stores = getAllStores();
    
    // 只持久化必要的状态，排除loading、error等临时状态
    const persistData = {
      app: {
        theme: stores.app.theme,
      },
      task: {
        filters: stores.task.filters,
      },
      action: {
        filters: stores.action.filters,
      },
    };
    localStorage.setItem('ducker-store-state', JSON.stringify(persistData));
  } catch (error) {
    console.warn('Failed to persist store state:', error);
  }
};

/**
 * 从localStorage恢复状态
 */
export const restoreStoreState = () => {
  if (typeof window === 'undefined') return;
  
  try {
    const savedState = localStorage.getItem('ducker-store-state');
    if (!savedState) return;
    
    const persistData = JSON.parse(savedState);
    
    if (persistData.app?.theme) {
      useAppStore.getState().setTheme(persistData.app.theme);
    }
    
    
    if (persistData.task?.filters) {
      useTaskStore.getState().setFilters(persistData.task.filters);
    }
    
    
    if (persistData.action?.filters) {
      useActionStore.getState().setFilters(persistData.action.filters);
    }
  } catch (error) {
    console.warn('Failed to restore store state:', error);
  }
};

/**
 * 订阅store变化并自动持久化
 */
export const setupAutoPersist = () => {
  if (typeof window === 'undefined') return;
  
  
  useAppStore.subscribe(
    (state) => state.theme,
    () => persistStoreState()
  );
  
  
  useTaskStore.subscribe(
    (state) => state.filters,
    () => persistStoreState()
  );
  
  useActionStore.subscribe(
    (state) => state.filters,
    () => persistStoreState()
  );
};

/**
 * 获取全局统计信息
 */
export const getGlobalStats = () => {
  const taskStats = useTaskStore.getState().getTaskStats();
  const actionStats = useActionStore.getState().getActionStats();
  
  return {
    tasks: taskStats,
    actions: actionStats,
    summary: {
      totalTasks: taskStats.total,
      completedTasks: taskStats.completed,
      totalActions: actionStats.total,
      completionRate: taskStats.completionRate,
    },
  };
};

/**
 * 清除所有错误状态
 */
export const clearAllErrors = () => {
  
  useAppStore.getState().clearError();
  useTaskStore.getState().clearError();
  useActionStore.getState().clearError();
};

/**
 * 检查是否有任何store正在加载
 */
export const isAnyStoreLoading = () => {
  const stores = getAllStores();
  return stores.app.loading || stores.task.loading || stores.action.loading || stores.action.executing;
};

/**
 * 获取所有错误信息
 */
export const getAllErrors = () => {
  const stores = getAllStores();
  const errors = [];
  
  if (stores.app.error) errors.push({ store: 'app', error: stores.app.error });
  if (stores.task.error) errors.push({ store: 'task', error: stores.task.error });
  if (stores.action.error) errors.push({ store: 'action', error: stores.action.error });
  
  return errors;
};

// 初始化
if (typeof window !== 'undefined') {
  // 恢复持久化状态
  restoreStoreState();
  
  // 设置自动持久化
  setupAutoPersist();
}