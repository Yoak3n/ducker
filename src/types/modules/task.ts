import type{ Action } from "./action";

export interface Task {
  id: number
  title: string
  completed: boolean
  actions?: Action[]
  auto?: boolean
  create_at: Date
  due_at?: Date
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

export interface TasksData {
  today: Task[];
  weekly: WeeklyTasks;
  monthly: Task[];
}