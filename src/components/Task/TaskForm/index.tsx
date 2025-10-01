import React, { useState, useEffect } from 'react';
import { emit } from '@tauri-apps/api/event';

import ActionSelect from '@/components/Action/ActionSelect';
import DatetimePicker from '@/components/Date/DatetimePicker';
import { Label } from '@/components/ui/label';
import { Collapsible } from '@radix-ui/react-collapsible';
import { CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Checkbox } from '@/components/ui/checkbox';


import { formatDatetime } from '@/utils';
import type { Task, TaskData, Action, Period, PeriodicTaskData } from '@/types';
import { create_periodic_task, update_periodic_task } from '@/api/modules/task';
import { destroyWindow } from '@/api/modules/window';

import './index.css';

interface TaskModalProps {
  onSave: (taskData: TaskData) => void;
  task?: Task; // 编辑模式时传入任务数据
  parentTask?: Task; // 创建子任务时传入父任务
}

interface TaskFormData {
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


const reminderOptions = [
  { value: 'ontime', label: '准时提醒', offset: 0 },
  { value: '5min', label: '提前5分钟', offset: 5 * 60 * 1000 },
  { value: '15min', label: '提前15分钟', offset: 15 * 60 * 1000 },
  { value: '30min', label: '提前30分钟', offset: 30 * 60 * 1000 },
  { value: '1hour', label: '提前1小时', offset: 60 * 60 * 1000 },
  { value: '2hour', label: '提前2小时', offset: 2 * 60 * 60 * 1000 },
  { value: '1day', label: '提前1天', offset: 24 * 60 * 60 * 1000 },
  { value: '3day', label: '提前3天', offset: 3 * 24 * 60 * 60 * 1000 },
  { value: '1week', label: '提前1周', offset: 7 * 24 * 60 * 60 * 1000 },
] as const;

const periodOptions = [
  { value: 0 as Period, label: '启动时执行', description: '应用启动时执行一次' }, // Period.OnStart
  { value: 100 as Period, label: '每天启动时执行一次', description: '每天启动时执行一次' }, // Period.OnceStarted
  { value: 1 as Period, label: '每日执行', description: '每天执行一次' }, // Period.Daily
  { value: 7 as Period, label: '每周执行', description: '每周执行一次' }, // Period.Weekly
  { value: 30 as Period, label: '每月执行', description: '每月执行一次' }, // Period.Monthly

] as const;


export default function TaskModal({ onSave, task, parentTask }: TaskModalProps) {
  const [formData, setFormData] = useState<TaskFormData>(initialFormData);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [isActionSelectOpen, setIsActionSelectOpen] = useState(false);
  const [isPeriodic, setIsPeriodic] = useState(false);

  // 处理键盘事件
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        if (isActionSelectOpen) {
          setIsActionSelectOpen(false);
        }
      }
    };

    if (isActionSelectOpen) {
      document.addEventListener('keydown', handleKeyDown);
      return () => document.removeEventListener('keydown', handleKeyDown);
    }
  }, [isActionSelectOpen]);

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
      return reminderDate.toISOString().slice(0, 16);
    } catch (error) {
      console.warn('计算提醒时间失败:', error);
      return '';
    }
  };

  // 初始化表单数据
  useEffect(() => {
    if (task) {
      // 编辑模式
      const dueToStr = task.due_to ? new Date(task.due_to).toISOString().slice(0, 16) : '';
      console.log("due:", dueToStr)
      const reminderStr = task.reminder ? new Date(task.reminder).toISOString().slice(0, 16) : '';
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
      console.log("formData:", formData)
    } else {
      // 创建模式
      const newFormData = { ...initialFormData };
      if (parentTask) {
        newFormData.parent_id = parentTask.id;
      }
      setFormData(newFormData);
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

  const handleInputChange = (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => {
    setFormData((prev) => ({ 
      ...prev, 
      [field]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log('提交表单数据:', formData);
    
    // 将TaskFormData转换为TaskData
    const taskData: TaskData = {
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
    if (task) {
      taskData.id = task.id;
      taskData.created_at = task.created_at;
    }

    // 如果是周期任务，创建周期任务记录
    if (isPeriodic) {
      try {
        // 创建周期任务需要先有任务ID，这里假设onSave会返回创建的任务
        // 实际使用中可能需要调整这个逻辑
        const periodicTaskData: PeriodicTaskData = {
          name: formData.name,
          interval: formData.periodicInterval,
          task: taskData,
        };
        task ? await update_periodic_task(task.id!, periodicTaskData) : await create_periodic_task(periodicTaskData);
        emit('task-changed', {timestamp: Date.now() })
      } catch (error) {
      }
    }else{
      onSave(taskData);
    }

    destroyWindow('task');
  };

  const handleClose = () => {
    setFormData(initialFormData);
    setShowAdvanced(false);
    destroyWindow('task');
  };

  return (
    <>
      <div className="text-xl font-semibold text-gray-800 px-5 py-3 border-b border-gray-200 cursor-move" data-tauri-drag-region>
        {task ? '编辑任务' : parentTask ? '创建子任务' : '创建任务'}
      </div>

      <form onSubmit={handleSubmit} className="p-5 overflow-y-auto flex-1 flex flex-col gap-4 scrollbar-none">
        {/* 基本信息区域 */}
        <div className="bg-gradient-to-br from-slate-50 to-slate-100 border border-slate-300 rounded-lg p-4 transition-all duration-200 hover:border-slate-400 hover:shadow-md">
          <div className="flex items-center gap-2 mb-3 pb-2 border-b border-gray-200 bg-white/80 rounded px-3 py-2 -mx-2">
            <span className="material-symbols-outlined text-slate-800 text-lg">edit</span>
            <h3 className="m-0 text-sm font-semibold text-gray-800">基本信息</h3>
          </div>

          {/* 任务状态、标题和自动执行在同一行 */}
          <div className={`grid gap-3 mb-4 items-end ${task ? 'grid-cols-[auto_1fr_auto]' : 'grid-cols-[1fr_auto]'}`}>
            {/* 任务完成状态 - 仅在编辑模式时显示 */}
            {task && (
              <div className="flex flex-col gap-2 min-w-[100px]">
                <label className="text-sm font-medium text-gray-600 m-0">任务状态</label>
                <div className='flex items-center gap-2 px-3 py-2.5 bg-white border border-gray-200 rounded-lg transition-all duration-200 cursor-pointer hover:bg-gray-50 hover:border-gray-300 hover:-translate-y-0.5 hover:shadow-lg h-10 justify-center'>
                  <Checkbox
                    checked={formData.completed}
                    onCheckedChange={(v) => handleInputChange('completed', v)}
                  />
                  <label className="text-sm cursor-pointer select-none">已完成</label>
                </div>
              </div>
            )}
            
            {/* 任务标题 */}
            <div className="flex-1 min-w-0">
              <label htmlFor="title" className="block mb-2 font-medium text-gray-600 text-sm">任务标题 *</label>
              <input
                type="text"
                id="title"
                value={formData.name}
                onChange={(e) => handleInputChange('name', e.target.value)}
                placeholder="输入任务标题..."
                autoComplete='off'
                required
                autoFocus
                className="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm transition-all duration-200 bg-white focus:outline-none focus:border-blue-500 focus:shadow-[0_0_0_3px_rgba(59,130,246,0.1)] h-10 placeholder:text-gray-400"
              />
            </div>
            
            {/* 自动执行 */}
            <div className="flex flex-col gap-2 min-w-[100px]">
              <label className="text-sm font-medium text-gray-600 m-0">自动执行</label>
              <div className='flex items-center gap-2 px-3 py-2.5 bg-slate-50 border border-slate-200 rounded-lg transition-all duration-200 cursor-pointer h-10 justify-center hover:bg-slate-100 hover:border-slate-300 hover:-translate-y-0.5 hover:shadow-lg'>
                <Checkbox
                  checked={formData.auto}
                  onCheckedChange={(v) => handleInputChange('auto', v)}
                />
                <span className="text-sm font-medium text-slate-600 m-0 cursor-pointer select-none">启用</span>
              </div>
            </div>
          </div>

          {parentTask && (
            <div className="flex items-center gap-2 px-3 py-2 bg-sky-50 border border-sky-200 rounded-md mb-3 text-sky-700 text-sm">
              <span className="material-symbols-outlined">subdirectory_arrow_right</span>
              <span>父任务: {parentTask.name}</span>
            </div>
          )}

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div className="mb-3">
              <label htmlFor="task-value" className="block mb-2 font-medium text-gray-600 text-sm">任务价值</label>
              <input
                id="task-value"
                type="number"
                value={formData.value || ''}
                onChange={(e) => handleInputChange('value', e.target.value ? Number(e.target.value) : undefined)}
                placeholder="请输入任务价值（可选）"
                min="0"
                step="1"
                className="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm transition-all duration-200 bg-white focus:outline-none focus:border-blue-500 focus:shadow-[0_0_0_3px_rgba(59,130,246,0.1)] h-10 placeholder:text-gray-400"
              />
            </div>
            <div className="mb-3">
              <label htmlFor="parent-task" className="block mb-2 font-medium text-gray-600 text-sm">父任务ID</label>
              <input
                id="parent-task"
                type="text"
                value={formData.parent_id || ''}
                onChange={(e) => handleInputChange('parent_id', e.target.value || undefined)}
                placeholder="请输入父任务ID（可选）"
                className="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm transition-all duration-200 bg-white focus:outline-none focus:border-blue-500 focus:shadow-[0_0_0_3px_rgba(59,130,246,0.1)] h-10 placeholder:text-gray-400"
              />
            </div>
          </div>
        </div>

        {/* 时间设置区域 */}
        <div className="bg-gradient-to-br from-purple-50 to-purple-100 border border-purple-200 rounded-lg p-4 transition-all duration-200 hover:border-purple-300 hover:shadow-md">
          <div className="flex items-center gap-2 mb-3 pb-2 border-b border-gray-200 bg-white/80 rounded px-3 py-2 -mx-2">
            <span className="material-symbols-outlined text-purple-600 text-lg">schedule</span>
            <h3 className="m-0 text-sm font-semibold text-gray-800">时间设置</h3>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div className="mb-3">
              <DatetimePicker
                datetime={formData.due_to}
                setDatetime={(datetime) => handleInputChange('due_to', datetime)}
              />
            </div>
            <div className="mb-3">
              <Label className='py-1'>提醒设置</Label>
              <Select
                value={formData.reminderOffset}
                onValueChange={(value) => handleInputChange('reminderOffset', value)}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="选择提醒设置" />
                </SelectTrigger>
                <SelectContent>
                  {reminderOptions.map(option => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </div>

        {/* 周期设置区域 */}
        <div className="bg-gradient-to-br from-slate-50 to-slate-100 border border-slate-200 rounded-lg p-4 transition-all duration-200 hover:border-slate-300 hover:shadow-md">
          <div className="flex items-center gap-2 mb-3 pb-2 border-b border-gray-200 bg-white/80 rounded px-3 py-2 -mx-2">
            <span className="material-symbols-outlined text-purple-500 text-lg">repeat</span>
            <h3 className="m-0 text-sm font-semibold text-gray-800">周期设置</h3>
          </div>

          <div className="mb-3">
            <div className="flex flex-col gap-2">
              <div className='flex flex-row items-center gap-2'>
                <Checkbox
                  className='cursor-pointer'
                  checked={isPeriodic}
                  onCheckedChange={(v) => setIsPeriodic(v as boolean)}
                />
                <label className="font-medium text-gray-600 cursor-default">设置为周期任务</label>
              </div>
              <span className="text-sm text-gray-500 italic ml-6">启用后，任务将按设定的周期自动重复执行</span>
            </div>
          </div>

          {isPeriodic && (
            <div className="mt-4 pt-4 border-t border-gray-200 animate-[slideDown_0.3s_ease-out] bg-white/50 rounded-md p-4">
              <div className="mb-3">
                <div className="mb-3">
                  <Label className='py-1'>执行周期</Label>
                  <Select
                    value={formData.periodicInterval.toString()}
                    onValueChange={(value) => handleInputChange('periodicInterval', Number(value) as Period)}
                  >
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="选择执行周期" />
                    </SelectTrigger>
                    <SelectContent>
                      {periodOptions.map(option => (
                        <SelectItem key={option.value} value={option.value.toString()}>
                          <div className="flex gap-1">
                            <span className="font-medium text-gray-600">{option.label}</span>
                            <span className="text-sm text-gray-500">{option.description}</span>
                          </div>
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* 高级设置区域 */}
        <div className="mt-2 border-t border-gray-200 pt-4">
          <Collapsible>
            <CollapsibleTrigger onClick={() => setShowAdvanced(!showAdvanced)}>
              <div className="flex items-center justify-between cursor-pointer w-full px-3 py-2 text-gray-500 text-sm font-medium transition-all duration-200 bg-slate-50 border border-slate-200 rounded-md hover:text-gray-600 hover:bg-slate-100 hover:border-slate-300">
                <div className="flex items-center gap-2">
                  <span className="material-symbols-outlined">
                    {showAdvanced ? 'expand_less' : 'expand_more'}
                  </span>
                  高级设置
                </div>
              </div>
            </CollapsibleTrigger>
            <CollapsibleContent>
              <div className="bg-slate-50 border border-slate-200 rounded-lg p-3 mt-2">
                <div className="flex justify-between items-center">
                  <div className="flex items-center gap-2">
                    <span className="material-symbols-outlined text-slate-600 text-lg">settings</span>
                    <h3 className="m-0 text-sm font-semibold text-slate-700">关联动作</h3>
                    <span className="bg-blue-500 text-white text-xs font-semibold px-2 py-1 rounded-full min-w-5 text-center">{formData.actions.length}</span>
                  </div>
                  <button
                    type="button"
                    className="flex items-center gap-2 px-3 py-2 bg-blue-500 text-white border-none rounded-md text-sm font-medium cursor-pointer transition-all duration-200 hover:bg-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)]"
                    onClick={() => setIsActionSelectOpen(true)}
                  >
                    <span className="material-symbols-outlined text-sm">add_circle</span>
                    添加动作
                  </button>
                </div>

                {formData.actions.length === 0 && (
                  <div className="flex flex-col items-center py-6 px-3 text-center text-slate-600">
                    <span className="material-symbols-outlined text-4xl text-slate-300 mb-2">psychology_alt</span>
                    <p className="m-0 mb-2 text-sm font-medium text-slate-700">暂无关联动作</p>
                    <span className="text-sm text-slate-500">点击上方按钮添加动作</span>
                  </div>
                )}

                {/* 显示已选择的 Actions */}
                {formData.actions.length > 0 && (
                  <div className="mt-3 p-3 bg-gray-50 rounded-md border border-gray-200">
                    <h4 className="m-0 mb-2 text-sm font-semibold text-gray-700">已选择的动作 ({formData.actions.length})</h4>
                    <div className="flex flex-col gap-2">
                      {formData.actions.map((action, index) => (
                        <div key={action.id} className="flex items-center gap-2 p-2 bg-white border border-gray-200 rounded text-sm">
                          <span className="font-semibold text-gray-500 min-w-5">{index + 1}.</span>
                          <span className="font-medium text-gray-800 flex-1">{action.name}</span>
                          <span className="text-gray-500 text-xs bg-gray-200 px-1 rounded">({action.type})</span>
                          <button
                            type="button"
                            className="flex items-center justify-center w-5 h-5 border-none bg-red-500 text-white rounded cursor-pointer transition-colors duration-200 hover:bg-red-600"
                            onClick={() => {
                              const newActions = formData.actions.filter((_, i) => i !== index);
                              handleInputChange('actions', newActions);
                            }}
                          >
                            <span className="material-symbols-outlined text-xs">close</span>
                          </button>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </CollapsibleContent>
          </Collapsible>
        </div>

        <div className="flex justify-end gap-3 mt-4 pt-4 border-t border-gray-200">
          <button type="button" className="px-4 py-2 rounded-md text-sm font-medium cursor-pointer transition-all duration-200 border min-w-[70px] bg-white text-gray-500 border-gray-300 hover:bg-gray-50 hover:text-gray-600 hover:border-gray-400" onClick={handleClose}>
            取消
          </button>
          <button 
            type="submit" 
            className="px-4 py-2 rounded-md text-sm font-medium cursor-pointer transition-all duration-200 border min-w-[70px] bg-blue-500 text-white border-blue-500 hover:bg-blue-600 hover:border-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)] disabled:bg-gray-400 disabled:border-gray-400 disabled:cursor-not-allowed disabled:opacity-60 disabled:transform-none disabled:shadow-none" 
            disabled={
              !formData.name.trim() || 
              (isPeriodic && !formData.periodicInterval)
            }
          >
            {task ? '保存' : '创建'}
          </button>
        </div>
      </form>

      {/* 动作选择模态框 */}
      {isActionSelectOpen && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[9999] backdrop-blur-sm animate-[fadeIn_0.2s_ease-out]">
          <div className="bg-white rounded-xl shadow-[0_25px_50px_-12px_rgba(0,0,0,0.25)] w-[90%] max-w-[800px] max-h-[80vh] flex flex-col overflow-hidden animate-[slideIn_0.3s_ease-out]">
            <div className="flex items-center justify-between px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-slate-50 to-gray-100 text-slate-800">
              <h2 className="m-0 text-xl font-semibold flex items-center gap-2 text-slate-700">
                <span className="material-symbols-outlined text-2xl text-slate-600">psychology_alt</span>
                选择关联动作
              </h2>
              <button
                className="flex items-center justify-center w-8 h-8 border-none bg-slate-600/10 text-slate-600 rounded-lg cursor-pointer transition-all duration-200 hover:bg-slate-600/20 hover:text-slate-700 hover:scale-105"
                onClick={() => setIsActionSelectOpen(false)}
              >
                <span className="material-symbols-outlined text-xl">close</span>
              </button>
            </div>
            <div className="flex-1 p-6 overflow-y-auto min-h-0">
              <ActionSelect
                selectedActions={formData.actions}
                onActionsChange={(actions) => handleInputChange('actions', actions)}
                multiSelect={true}
                maxHeight='50vh'
              />
            </div>
            <div className="flex justify-end gap-3 px-6 py-4 border-t border-gray-200 bg-gray-50">
              <button 
                className="px-5 py-2 border border-gray-300 bg-white text-gray-500 rounded-lg text-sm font-medium cursor-pointer transition-all duration-200 hover:bg-gray-50 hover:border-gray-400 hover:text-gray-600"
                onClick={() => setIsActionSelectOpen(false)}
              >
                取消
              </button>
              <button 
                className="px-5 py-2 border border-blue-500 bg-blue-500 text-white rounded-lg text-sm font-medium cursor-pointer transition-all duration-200 flex items-center gap-2 hover:bg-blue-600 hover:border-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)]"
                onClick={() => setIsActionSelectOpen(false)}
              >
                <span className="material-symbols-outlined text-base">check</span>
                确认选择
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
