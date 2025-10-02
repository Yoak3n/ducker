 
export interface TaskFormData {
  name: string;
  value?: number;
  completed: boolean;
  auto: boolean;
  due_to: string; // datetime-local格式字符串
  reminder: string; // datetime-local格式字符串
  reminderOffset: string; // 提醒时间偏移量选择器值
  parent_id?: string;
  actions: Action[]; // 表单中使用完整Action对象
  // 周期设置相关字段
  periodicInterval: Period; // 周期间隔
}


export type InputHandle = (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => void;