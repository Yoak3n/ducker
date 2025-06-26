import React, { useState, useEffect } from 'react';
import type { Task, TaskData, Action } from '@/types';
import './index.css';
import ActionSelect from '@/components/Action/ActionSelect';

interface TaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (taskData: TaskData) => void;
  task?: Task | null; // 编辑模式时传入任务数据
  parentTask?: Task | null; // 创建子任务时传入父任务
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
}

const initialFormData: TaskFormData = {
  name: '',
  value: undefined,
  completed: false,
  auto: false,
  due_to: '',
  reminder: '',
  reminderOffset: 'ontime', // 默认准时
  parent_id: undefined,
  actions: []
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
  { value: '1week', label: '提前1周', offset: 7 * 24 * 60 * 60 * 1000 }
] as const;


export default function TaskModal({ isOpen, onClose, onSave, task, parentTask }: TaskModalProps) {
  const [formData, setFormData] = useState<TaskFormData>(initialFormData);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [isActionSelectOpen, setIsActionSelectOpen] = useState(false);

  // 处理键盘事件
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        if (isActionSelectOpen) {
          setIsActionSelectOpen(false);
        } else if (isOpen) {
          // setIsOpen(false);
        }
      }
    };

    if (isOpen || isActionSelectOpen) {
      document.addEventListener('keydown', handleKeyDown);
      return () => document.removeEventListener('keydown', handleKeyDown);
    }
  }, [isOpen, isActionSelectOpen]);

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
    if (!dueToStr) return '';

    try {
      const dueDate = new Date(dueToStr);
      if (isNaN(dueDate.getTime())) return '';

      const option = reminderOptions.find(opt => opt.value === offsetValue);
      if (!option) return '';

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
      const reminderStr = task.reminder ? new Date(task.reminder).toISOString().slice(0, 16) : '';
      const reminderOffset = task.due_to && task.reminder
        ? calculateReminderOffset(new Date(task.due_to), new Date(task.reminder))
        : 'ontime';

      // 从Task转换为TaskFormData
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
        actions: task.actions || []
      });
    } else {
      // 创建模式
      const newFormData = { ...initialFormData };
      if (parentTask) {
        newFormData.parent_id = parentTask.id;
      }
      setFormData(newFormData);
    }
  }, [task, parentTask, isOpen]);

  // 当截止时间或提醒偏移量改变时，自动计算提醒时间
  useEffect(() => {
    if (formData.due_to && formData.reminderOffset) {
      const newReminderTime = calculateReminderTime(formData.due_to, formData.reminderOffset);
      if (newReminderTime !== formData.reminder) {
        setFormData(prev => ({ ...prev, reminder: newReminderTime }));
      }
    }
  }, [formData.due_to, formData.reminderOffset]);

  const handleInputChange = (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[]) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    // 将TaskFormData转换为TaskData
    const taskData: TaskData = {
      name: formData.name,
      value: formData.value,
      completed: formData.completed,
      auto: formData.auto,
      parent_id: formData.parent_id,
      due_to: formData.due_to ? new Date(formData.due_to) : undefined,
      reminder: formData.reminder ? new Date(formData.reminder) : undefined,
      // 将Action[]转换为string[]
      actions: formData.actions.map(action => action.id!)
    };

    // 编辑模式下保留原有ID和创建时间
    if (task) {
      taskData.id = task.id;
      taskData.created_at = task.created_at;
    }

    onSave(taskData);
    onClose();
  };

  const handleClose = () => {
    setFormData(initialFormData);
    setShowAdvanced(false);
    onClose();
  };

  if (!isOpen) return null;
  return (
    <div className="task-modal-overlay" onClick={handleClose}>
      <div className="task-modal" onClick={(e) => e.stopPropagation()}>
        <div className="task-modal-header">
          <h2>
            {task ? '编辑任务' : parentTask ? '创建子任务' : '创建任务'}
          </h2>
          <button className="close-button" onClick={handleClose}>
            <span className="material-symbols-outlined">close</span>
          </button>
        </div>
        <form onSubmit={handleSubmit} className="task-modal-form">
          <div className="form-group">
            <label htmlFor="title">任务标题 *</label>
            <input
              type="text"
              id="title"
              value={formData.name}
              onChange={(e) => handleInputChange('name', e.target.value)}
              placeholder="输入任务标题..."
              autoComplete='off'
              required
              autoFocus
            />
          </div>

          <div className="form-group">
            <label htmlFor="task-value">任务价值</label>
            <input
              id="task-value"
              type="number"
              value={formData.value || ''}
              onChange={(e) => handleInputChange('value', e.target.value ? Number(e.target.value) : undefined)}
              placeholder="请输入任务价值（可选）"
              min="0"
              step="1"
            />
          </div>

          <div className="form-group">
            <label htmlFor="parent-task">父任务ID</label>
            <input
              id="parent-task"
              type="text"
              value={formData.parent_id || ''}
              onChange={(e) => handleInputChange('parent_id', e.target.value || undefined)}
              placeholder="请输入父任务ID（可选）"
            />
          </div>

          {parentTask && (
            <div className="parent-task-info">
              <span className="material-symbols-outlined">subdirectory_arrow_right</span>
              <span>父任务: {parentTask.name}</span>
            </div>
          )}

          <div className="form-row">
            <div className="form-group">
              <label htmlFor="due_to">截止时间</label>
              <input
                type="datetime-local"
                id="due_to"
                value={formData.due_to}
                onChange={(e) => handleInputChange('due_to', e.target.value)}
              />
            </div>
            <div className="form-group">
              <label htmlFor="reminder-offset">提醒设置</label>
              <select
                id="reminder-offset"
                value={formData.reminderOffset}
                onChange={(e) => handleInputChange('reminderOffset', e.target.value)}
                className="reminder-offset-select"
              >
                {reminderOptions.map(option => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className="form-group checkbox-group">
            {task && <label className="checkbox-label">
              <input
                type="checkbox"
                checked={formData.completed}
                onChange={(e) => handleInputChange('completed', e.target.checked)}
              />
              <span>已完成</span>
            </label>}
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={formData.auto}
                onChange={(e) => handleInputChange('auto', e.target.checked)}
              />
              <span>自动执行</span>
            </label>
          </div>

          <div className="advanced-section">
            <button
              type="button"
              className="advanced-toggle"
              onClick={() => setShowAdvanced(!showAdvanced)}
            >
              <span className="material-symbols-outlined">
                {showAdvanced ? 'expand_less' : 'expand_more'}
              </span>
              高级设置
            </button>

            {showAdvanced && (
              <div className="advanced-content">
                <div className="actions-section">
                  <div className="actions-header">
                    <div className="actions-title">
                      <span className="material-symbols-outlined">settings</span>
                      <h3>关联动作</h3>
                      <span className="actions-count">{formData.actions.length}</span>
                    </div>
                    <button
                      type="button"
                      className="add-action-button"
                      onClick={() => setIsActionSelectOpen(true)}
                    >
                      <span className="material-symbols-outlined">add_circle</span>
                      添加动作
                    </button>
                  </div>

                  {formData.actions.length === 0 && (
                    <div className="actions-empty-state">
                      <span className="material-symbols-outlined">psychology_alt</span>
                      <p>暂无关联动作</p>
                      <span className="empty-hint">点击上方按钮添加动作</span>
                    </div>
                  )}
                </div>

                {/* 显示已选择的 Actions */}
                {formData.actions.length > 0 && (
                  <div className="selected-actions">
                    <h4>已选择的动作 ({formData.actions.length})</h4>
                    <div className="actions-list">
                      {formData.actions.map((action, index) => (
                        <div key={action.id} className="action-item-preview">
                          <span className="action-order">{index + 1}.</span>
                          <span className="action-name">{action.name}</span>
                          <span className="action-type">({action.type})</span>
                          <button
                            type="button"
                            className="remove-action-btn"
                            onClick={() => {
                              const newActions = formData.actions.filter((_, i) => i !== index);
                              handleInputChange('actions', newActions);
                            }}
                          >
                            <span className="material-symbols-outlined">close</span>
                          </button>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}

          </div>

          <div className="form-actions">
            <button type="button" className="cancel-btn" onClick={handleClose}>
              取消
            </button>
            <button type="submit" className="save-btn" disabled={!formData.name.trim()}>
              {task ? '保存' : '创建'}
            </button>
          </div>
        </form>

      </div>

      {/* 独立的动作选择窗口 */}
      {isActionSelectOpen && (
        <div className="action-select-modal" onClick={() => setIsActionSelectOpen(false)}>
          <div className="action-select-modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="action-select-modal-header">
              <h2>选择动作</h2>
              <button className="close-button" onClick={() => setIsActionSelectOpen(false)}>
                <span className="material-symbols-outlined">close</span>
              </button>
            </div>
            <div className="action-select-modal-body">
              <ActionSelect
                selectedActions={formData.actions}
                onActionsChange={(actions) => handleInputChange('actions', actions)}
                multiSelect={true}
                maxHeight="50vh"
              />
            </div>
            <div className="action-select-modal-footer">
              <button
                type="button"
                className="cancel-btn"
                onClick={() => setIsActionSelectOpen(false)}
              >
                取消
              </button>
              <button
                type="button"
                className="confirm-btn"
                onClick={() => setIsActionSelectOpen(false)}
              >
                确认选择 ({formData.actions.length})
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
