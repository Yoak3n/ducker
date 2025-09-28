import type{ Action } from "./action";

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

export interface TaskData {
  id?: string,
  name: string
  value?: number
  completed: boolean
  auto?: boolean
  actions?: string[]
  parent_id?: string,
  created_at?: string
  due_to?: string
  reminder?: string
}

// 周期任务相关类型定义
export const Period = {
  OnStart: 0,
  Daily: 1,
  Weekly: 7,
  Monthly: 30,
  OnceStarted: 100,
} as const;

export type Period = typeof Period[keyof typeof Period];

export interface PeriodicTask{
  id?: number;
  name: string;
  interval: Period;
  task: Task;
  last_run?: number;
}

export interface PeriodicTaskData {
  name: string;
  interval: Period;
  task: TaskData;
}

export interface PeriodicTaskRecord {
  id?: string;
  name: string;
  interval: Period;
  last_run?: number;
}

export interface WeeklyTasks {
  monday: Task[];
  tuesday: Task[];
  wednesday: Task[];
  thursday: Task[];
  friday: Task[];
  saturday: Task[];
  sunday: Task[];
}

export interface TaskMocks{
  today: Task[];
  weekly: WeeklyTasks;
  monthly: Task[];
}

