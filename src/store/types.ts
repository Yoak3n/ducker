// 统一类型定义
import type { Task, Action, TaskData, CreateActionData, UpdateActionData, Config, PeriodicTask } from "@/types";
export type { Task, Action, TaskData, CreateActionData, UpdateActionData }

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

export interface ConfigFilters {}

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
  periodicTasks: PeriodicTask[];
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

export interface Live2DState extends BaseState {
  show: boolean;
  modelPath: string;
  width: number;
  height: number;
  position: { x: number; y: number };
  scale: number;
}

export interface ConfigState extends BaseState, Config {
  filters: ConfigFilters;
}