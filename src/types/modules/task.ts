import type{ Action } from "./action";

// 基础任务接口
export interface Task {
  id: string,
  name: string
  value: number
  completed: boolean
  auto?: boolean
  actions?: Action[]
  children?: Task[]
  created_at: string
  due_to?: string
  reminder?: string
}

// 任务数据接口 - 使用工具类型优化
export interface TaskData extends 
  Omit<Task, 'id' | 'actions' | 'children' | 'value' | 'created_at'>,
  Partial<Pick<Task, 'id' | 'value' | 'created_at'>> {
  actions?: string[]  // 在TaskData中actions是字符串数组
  parent_id?: string  // TaskData特有字段
}

// 常用的工具类型定义
export type CreateTaskData = Omit<TaskData, 'id' | 'created_at'>
export type UpdateTaskData = Partial<Omit<TaskData, 'id'>>
export type TaskFormData = Omit<TaskData, 'id' | 'created_at' | 'parent_id'>

// 任务字段选择器类型
export type TaskBasicInfo = Pick<Task, 'id' | 'name' | 'completed'>
export type TaskTimeInfo = Pick<Task, 'created_at' | 'due_to' | 'reminder'>
export type TaskOptionalFields = Pick<Task, 'auto' | 'actions' | 'children'>

// 周期任务相关类型定义
export const Period = {
  OnStart: 0,
  Daily: 1,
  Weekly: 7,
  Monthly: 30,
  OnceStarted: 100,
} as const;

export type Period = typeof Period[keyof typeof Period];

// 周期任务基础接口
export interface PeriodicTaskBase {
  name: string;
  interval: Period;
  last_period?: number;
  next_period?: number;
}

// 完整的周期任务接口
export interface PeriodicTask extends PeriodicTaskBase {
  id?: number;
  task: Task;
}

// 周期任务数据接口 - 用于创建和更新
export interface PeriodicTaskData extends Omit<PeriodicTaskBase, 'last_period' | 'next_period'> {
  task: TaskData;
}

// 周期任务记录接口 - 数据库记录格式
export interface PeriodicTaskRecord extends PeriodicTaskBase {
  id?: string;
}

// 周期任务工具类型
export type CreatePeriodicTaskData = Omit<PeriodicTaskData, 'id'>
export type UpdatePeriodicTaskData = Partial<PeriodicTaskData>
export type PeriodicTaskInfo = Pick<PeriodicTask, 'id' | 'name' | 'interval'>

// 星期任务类型 - 使用Record简化定义
export type WeekDay = 'monday' | 'tuesday' | 'wednesday' | 'thursday' | 'friday' | 'saturday' | 'sunday'
export type WeeklyTasks = Record<WeekDay, Task[]>

// 任务模拟数据接口
export interface TaskMocks {
  today: Task[];
  weekly: WeeklyTasks;
  monthly: Task[];
}

// 任务状态相关类型
export type TaskStatus = Pick<Task, 'completed'>
export type TaskWithStatus = Task & { status: 'pending' | 'completed' | 'overdue' }

// 批量操作类型
export type TaskIds = string[]
export type BulkTaskUpdate = Partial<Pick<Task, 'completed' | 'auto'>>
export type BulkTaskOperation = {
  ids: TaskIds;
  updates: BulkTaskUpdate;
}

// 任务查询和过滤类型
export type TaskFilter = {
  completed?: boolean;
  auto?: boolean;
  hasActions?: boolean;
  hasChildren?: boolean;
  dateRange?: {
    start?: string;
    end?: string;
  };
}

export type TaskSortField = keyof Pick<Task, 'name' | 'value' | 'created_at' | 'due_to'>
export type TaskSortOrder = 'asc' | 'desc'
export type TaskSort = {
  field: TaskSortField;
  order: TaskSortOrder;
}

// 任务统计类型
export type TaskStats = {
  total: number;
  completed: number;
  pending: number;
  overdue: number;
  totalValue: number;
  completedValue: number;
}

// 任务关系类型
export type TaskWithChildren = Task & Required<Pick<Task, 'children'>>
export type TaskWithParent = Task & { parent?: Task }
export type TaskHierarchy = TaskWithChildren & TaskWithParent

