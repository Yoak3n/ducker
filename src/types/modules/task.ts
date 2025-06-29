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

