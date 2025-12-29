import { type FormEvent, useState, useEffect, useCallback } from 'react';

import { formatDatetime } from '@/utils';
import type { Task, TaskData, Action, Period } from '@/types';
import { destroyWindow } from '@/api/modules/window';
import { useI18n } from '@/hooks/use-i18n';

import { reminderOptions } from './options';
import Advanced from './Advanced';
import type { TaskFormData } from './type.d';

import './index.css';
import Periodic from './Periodic';
import Time from './Time';
import Information from './Information';

interface TaskFormProps {
  onSave: (taskData: TaskData, periodic?: number) => void;
  task?: Task; // 编辑模式时传入任务数据
  parentTask?: Task; // 创建子任务时传入父任务
}

const initialFormData: TaskFormData = {
  name: '',
  value: undefined,
  completed: false,
  auto: false,
  due_to: '',
  reminder: '',
  reminderOffset: 'ontime', // 默认不提醒
  parent_id: undefined,
  actions: [],
  // 周期设置默认值
  periodicInterval: 1 as Period, // Period.Daily
};

export default function TaskForm({ onSave, task, parentTask }: TaskFormProps) {
  const [formData, setFormData] = useState<TaskFormData>(initialFormData);
  const [isPeriodic, setIsPeriodic] = useState(false);
  const [isEditMode, setIsEditMode] = useState(Boolean(task));
  const { t } = useI18n();

  const calculateReminderOffset = (dueDate: Date, reminderDate: Date): string => {
    try {
      const diff = dueDate.getTime() - reminderDate.getTime();
      const option = reminderOptions.find(opt => opt.offset === diff);
      return option ? option.value : 'ontime';
    } catch (error) {
      console.warn('计算提醒偏移量失败:', error);
      return 'ontime';
    }
  };

  const calculateReminderTime = (dueToStr: string, offsetValue: string): string => {
    if (!dueToStr || offsetValue === 'none') return '';

    try {
      const dueDate = new Date(dueToStr);
      if (isNaN(dueDate.getTime())) return '';

      const option = reminderOptions.find(opt => opt.value === offsetValue);
      if (!option || option.offset < 0) return '';

      const reminderDate = new Date(dueDate.getTime() - option.offset);
      return formatDatetime(reminderDate);
    } catch (error) {
      console.warn('计算提醒时间失败:', error);
      return '';
    }
  };

  // 辅助函数：将日期转换为 DatetimePicker 期望的格式 "YYYY-MM-DD HH:mm:ss"
  const formatDateForPicker = (dateStr: string): string => {
    if (!dateStr) return '';
    try {
      const date = new Date(dateStr);
      if (isNaN(date.getTime())) return '';
      return formatDatetime(date);
    } catch (error) {
      console.warn('日期格式转换失败:', error);
      return '';
    }
  };

  // 初始化表单数据
  useEffect(() => {
    if (task) {
      // 编辑模式
      const dueToStr = task.due_to ? formatDateForPicker(task.due_to) : '';
      const reminderStr = task.reminder ? formatDateForPicker(task.reminder) : '';
      const reminderOffset = task.due_to && task.reminder
        ? calculateReminderOffset(new Date(task.due_to), new Date(task.reminder))
        : 'ontime';      // 从Task转换为TaskFormData
      setFormData({
        name: task.name,
        value: task.value,
        completed: task.completed,
        auto: task.auto || false,
        due_to: dueToStr,
        reminder: reminderStr,
        reminderOffset: reminderOffset,
        // Task类型中没有parent_id字段，需要从其他地方获取或保持undefined
        parent_id: undefined,
        actions: task.actions || [],
        // 周期任务相关字段
        periodicInterval: 1 as Period,
      });

      // 如果任务有 periodic 字段，设置为周期任务
      if (task.periodic) {
        setIsPeriodic(true);
      }
      console.log("编辑模式 - 格式化后的时间数据:", { dueToStr, reminderStr });
      setIsEditMode(true);
    } else {
      // 创建模式
      const newFormData = { ...initialFormData };
      if (parentTask) {
        newFormData.parent_id = parentTask.id;
      }
      setFormData(newFormData);
      console.log('创建模式 - 初始化表单数据:', newFormData);
      setIsEditMode(false);
    }
  }, [task, parentTask]);

  // 当截止时间或提醒偏移量改变时，自动计算提醒时间
  useEffect(() => {
    if (formData.due_to && formData.reminderOffset) {
      const newReminderTime = calculateReminderTime(formData.due_to, formData.reminderOffset);
      if (newReminderTime !== formData.reminder) {
        setFormData(prev => ({ ...prev, reminder: newReminderTime }));
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [formData.reminderOffset, formData.due_to]);

  const handleInputChange = useCallback((field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => {
    setFormData((prev) => ({
      ...prev,
      [field]: value,
    }));
  }, []);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    // 将TaskFormData转换为TaskData
    let taskData: TaskData = {
      name: formData.name,
      value: formData.value,
      completed: formData.completed,
      auto: formData.auto,
      parent_id: formData.parent_id,
      due_to: formData.due_to ? formatDatetime(new Date(formData.due_to)) : undefined,
      reminder: formData.reminder ? formatDatetime(new Date(formData.reminder)) : undefined,
      // 将Action[]转换为string[]
      actions: formData.actions.map(action => action.id!)
    };

    // 编辑模式下保留原有ID和创建时间
    if (isEditMode && task) {
      taskData.id = task.id;
      taskData.created_at = task.created_at;
    }

    // 如果是周期任务，创建周期任务记录
    onSave(taskData,isPeriodic?formData.periodicInterval:undefined);
    destroyWindow('task');
  };

  const handleClose = () => {
    setFormData(initialFormData);
    destroyWindow('task');
  };

  return (
    <>
      <div className="text-xl font-semibold text-gray-800 px-5 py-3 border-b border-gray-200 cursor-move" data-tauri-drag-region>
        {task ? t("Edit Task") : parentTask ? t("Create Child Task") : t("Create Task")}
      </div>

      <form onSubmit={handleSubmit} className="p-5 overflow-y-auto flex-1 flex flex-col gap-4 scrollbar-none">
        {/* 基本信息区域 */}
        <Information
          editing={task ? true : false}
          completed={formData.completed}
          name={formData.name}
          auto={formData.auto}
          value={formData.value}
          parentTask={parentTask}
          handleInputChange={handleInputChange}
        />

        {/* 时间设置区域 */}
        <Time
          dueTo={formData.due_to}
          reminderOffset={formData.reminderOffset}
          handleInputChange={handleInputChange}
        />

        {/* 周期设置区域 */}
        <Periodic
          isPeriodic={isPeriodic}
          setIsPeriodic={setIsPeriodic}
          periodicInterval={formData.periodicInterval}
          handleInputChange={handleInputChange}
        />

        {/* 高级设置区域 */}
        <Advanced
          actionsData={formData.actions}
          handleInputChange={handleInputChange}
        />

        {/* 操作区 */}
        <div className="flex justify-end gap-3 mt-4 pt-4 border-t border-gray-200">
          <button type="button" className="px-4 py-2 rounded-md text-sm font-medium cursor-pointer transition-all duration-200 border min-w-[70px] bg-white text-gray-500 border-gray-300 hover:bg-gray-50 hover:text-gray-600 hover:border-gray-400" onClick={handleClose}>
            {t("Cancel")}
          </button>
          <button
            type="submit"
            className="px-4 py-2 rounded-md text-sm font-medium cursor-pointer transition-all duration-200 border min-w-[70px] bg-blue-500 text-white border-blue-500 hover:bg-blue-600 hover:border-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)] disabled:bg-gray-400 disabled:border-gray-400 disabled:cursor-not-allowed disabled:opacity-60 disabled:transform-none disabled:shadow-none"
            disabled={
              !formData.name.trim() ||
              (isPeriodic && formData.periodicInterval === undefined)
            }
          >
            {task ? t("Save") : t("Create")}
          </button>
        </div>
      </form>
    </>
  );
}
