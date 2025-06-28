# Zustand 状态管理系统

这是一个基于 Zustand 构建的轻量级、易维护的状态管理系统。

## 🚀 特性

- **轻量级**: 无需 Provider 包装，零配置
- **TypeScript 友好**: 完整的类型支持
- **高性能**: 精确订阅，避免不必要的重渲染
- **开发友好**: 集成 Redux DevTools
- **组件外使用**: 可在组件外部访问和修改状态
- **状态持久化**: 支持本地存储同步

## 📁 项目结构

```
src/store/
├── types.ts          # 类型定义
├── appStore.ts       # 应用全局状态
├── taskStore.ts      # 任务管理状态
├── actionStore.ts    # 动作管理状态
├── index.ts          # 统一导出入口
└── README.md         # 使用文档
```

## 🎯 快速开始

### 1. 基本使用

```tsx
import { useAppStore, useTaskStore, useActionStore } from '@/store';

function MyComponent() {
  // 订阅应用状态
  const { theme, setTheme, loading } = useAppStore();
  
  // 订阅任务状态
  const { tasks, createTask, deleteTask } = useTaskStore();
  
  // 订阅动作状态
  const { actions, executeAction } = useActionStore();
  
  return (
    <div>
      <p>当前主题: {theme}</p>
      <p>任务数量: {tasks.length}</p>
      <p>动作数量: {actions.length}</p>
    </div>
  );
}
```

### 2. 组件外使用

```tsx
import { appStore, taskStore, actionStore } from '@/store';

// 直接访问状态
const currentTheme = appStore.getState().theme;
const allTasks = taskStore.getState().tasks;

// 直接修改状态
appStore.getState().setTheme('dark');
taskStore.getState().createTask({ name: '新任务', completed: false });

// 订阅状态变化
const unsubscribe = taskStore.subscribe(
  (state) => state.tasks,
  (tasks) => console.log('任务列表更新:', tasks)
);
```

### 3. 状态持久化

```tsx
import { persistStoreState, restoreStoreState, setupAutoPersist } from '@/store';

// 手动保存状态
persistStoreState();

// 手动恢复状态
restoreStoreState();

// 自动持久化（推荐）
setupAutoPersist();
```

## 📊 Store 详解

### AppStore - 应用全局状态

```tsx
interface AppState {
  theme: 'light' | 'dark' | 'system';
  loading: boolean;
  error: string | null;
}

interface AppActions {
  setTheme: (theme: Theme) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
  reset: () => void;
}
```

**主要功能:**
- 主题管理（支持本地存储）
- 全局加载状态
- 全局错误处理

### TaskStore - 任务管理

```tsx
interface TaskState {
  tasks: Task[];
  currentTask: Task | null;
  loading: boolean;
  error: string | null;
  filters: TaskFilters;
}

interface TaskActions {
  // CRUD 操作
  fetchTasks: () => Promise<void>;
  createTask: (data: CreateTaskData) => Promise<void>;
  updateTask: (id: string, data: UpdateTaskData) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  
  // 便捷操作
  toggleTaskCompletion: (id: string) => Promise<void>;
  bulkUpdateTasks: (ids: string[], data: UpdateTaskData) => Promise<void>;
  bulkDeleteTasks: (ids: string[]) => Promise<void>;
  
  // 查询方法
  getTaskById: (id: string) => Task | undefined;
  getCompletedTasks: () => Task[];
  getFilteredTasks: () => Task[];
  getTaskStats: () => TaskStats;
}
```

**主要功能:**
- 任务 CRUD 操作
- 批量操作支持
- 智能过滤和搜索
- 统计信息计算

### ActionStore - 动作管理

```tsx
interface ActionState {
  actions: Action[];
  currentAction: Action | null;
  loading: boolean;
  error: string | null;
  executing: boolean;
  executionResults: ExecutionResult[];
  filters: ActionFilters;
}

interface ActionActions {
  // CRUD 操作
  fetchActions: () => Promise<void>;
  createAction: (data: CreateActionData) => Promise<void>;
  updateAction: (id: string, data: UpdateActionData) => Promise<void>;
  deleteAction: (id: string) => Promise<void>;
  
  // 执行操作
  executeSingleAction: (action: Action) => Promise<void>;
  executeBatchActions: (actions: Action[]) => Promise<void>;
  
  // 查询方法
  getActionById: (id: string) => Action | undefined;
  getActionsByType: (type: ActionType) => Action[];
  getFilteredActions: () => Action[];
  getActionStats: () => ActionStats;
}
```

**主要功能:**
- 动作 CRUD 操作
- 命令执行和 URL 打开
- 批量执行支持
- 执行结果跟踪

## 🛠 工具函数

```tsx
import {
  getAllStores,
  resetAllStores,
  persistStoreState,
  restoreStoreState,
  setupAutoPersist,
  getGlobalStats,
  clearAllErrors,
  isAnyStoreLoading,
  getAllErrors
} from '@/store';

// 获取所有 store 实例
const stores = getAllStores();

// 重置所有状态
resetAllStores();

// 获取全局统计
const stats = getGlobalStats();

// 检查是否有 store 正在加载
const loading = isAnyStoreLoading();

// 获取所有错误
const errors = getAllErrors();

// 清除所有错误
clearAllErrors();
```

## 🎨 最佳实践

### 1. 状态订阅优化

```tsx
// ❌ 避免订阅整个 store
const store = useTaskStore();

// ✅ 只订阅需要的状态
const tasks = useTaskStore(state => state.tasks);
const loading = useTaskStore(state => state.loading);
```

### 2. 异步操作处理

```tsx
const handleCreateTask = async () => {
  try {
    await createTask({ name: 'New Task', completed: false });
    toast.success('任务创建成功');
  } catch (error) {
    toast.error('任务创建失败');
  }
};
```

### 3. 错误处理

```tsx
useEffect(() => {
  if (error) {
    toast.error(error);
    clearError();
  }
}, [error, clearError]);
```

### 4. 状态重置

```tsx
// 组件卸载时重置状态
useEffect(() => {
  return () => {
    setCurrentTask(null);
    clearError();
  };
}, []);
```

## 🔧 开发调试

### Redux DevTools

安装 Redux DevTools 浏览器扩展后，可以在开发者工具中查看状态变化：

1. 打开浏览器开发者工具
2. 切换到 "Redux" 标签
3. 查看状态树和动作历史

### 状态重置

```tsx
// 开发时快速重置状态
if (process.env.NODE_ENV === 'development') {
  (window as any).resetStores = resetAllStores;
}

// 在控制台执行: resetStores()
```

## 📈 性能优化

- **精确订阅**: 只订阅需要的状态片段
- **批量更新**: 使用批量操作减少渲染次数
- **懒加载**: Store 按需创建和初始化
- **内存管理**: 及时清理不需要的状态

## 🔄 迁移指南

如果你正在从其他状态管理方案迁移，请参考：

1. **从 Context API**: 移除 Provider，直接使用 hooks
2. **从 Redux**: 简化 actions 和 reducers，使用直接的状态更新
3. **从 MobX**: 保持响应式思维，但使用更简单的 API

## 📝 示例

查看 `src/components/examples/StoreExample.tsx` 获取完整的使用示例。

访问 `/store-test` 路由查看在线演示。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个状态管理系统！