// 统一类型定义

// 基础实体类型
export interface Task {
  id: string;
  name: string;
  completed: boolean;
  value: number;
  createdAt?: string;
  updatedAt?: string;
}

export interface Action {
  id: string;
  name: string;
  desc: string;
  type: 'exec_command' | 'open_url' | 'open_file' | 'open_dir';
  command: string;
  args?: string[];
  url?: string;
  wait: number;
  createdAt?: string;
  updatedAt?: string;
}

// 创建数据类型（不包含id等自动生成字段）
export type CreateTaskData = Omit<Task, 'id' | 'createdAt' | 'updatedAt'>;
export type CreateActionData = Omit<Action, 'id' | 'createdAt' | 'updatedAt'>;
export type UpdateTaskData = Partial<CreateTaskData>;
export type UpdateActionData = Partial<CreateActionData>;

// 过滤器类型
export interface TaskFilters {
  completed?: boolean;
  search?: string;
  dateRange?: {
    start: Date;
    end: Date;
  };
}

export interface ActionFilters {
  type?: Action['type'];
  search?: string;
}

// 统计信息类型
export interface TaskStats {
  total: number;
  completed: number;
  pending: number;
  completionRate: number;
}

export interface ActionStats {
  total: number;
  byType: Record<Action['type'], number>;
}

// 执行结果类型
export interface ExecutionResult {
  actionId: string;
  success: boolean;
  output?: string;
  error?: string;
  executedAt: string;
}

// 主题类型
export type Theme = 'light' | 'dark' | 'system';

// 通用状态类型
export interface BaseState {
  loading: boolean;
  error: string | null;
}

// Store状态类型
export interface AppState extends BaseState {
  theme: Theme;
}

export interface TaskState extends BaseState {
  tasks: Task[];
  currentTask: Task | null;
  filters: TaskFilters;
}

export interface ActionState extends BaseState {
  actions: Action[];
  currentAction: Action | null;
  executing: boolean;
  executionResults: ExecutionResult[];
  filters: ActionFilters;
}