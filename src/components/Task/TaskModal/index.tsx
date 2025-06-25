import React, { useState, useEffect } from 'react';
import type { Task, Action } from '@/types';
import './index.css';
import ActionSelect from '@/components/Action/ActionSelect';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { Button } from '@/components/ui/button';

interface TaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (task: Partial<Task>) => void;
  task?: Task | null; // 编辑模式时传入任务数据
  parentTask?: Task | null; // 创建子任务时传入父任务
}

interface TaskFormData {
  name: string;
  completed: boolean;
  auto: boolean;
  due_to: string;
  reminder: string;
  actions: Action[];
}

const initialFormData: TaskFormData = {
  name: '',
  completed: false,
  auto: false,
  due_to: '',
  reminder: '',
  actions: []
};

const actionTypes = [
  { value: 'exec_command', label: '执行命令' },
  { value: 'open_file', label: '打开文件' },
  { value: 'open_dir', label: '打开目录' },
  { value: 'open_url', label: '打开链接' }
];

export default function TaskModal({ isOpen, onClose, onSave, task, parentTask }: TaskModalProps) {
  const [formData, setFormData] = useState<TaskFormData>(initialFormData);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [showActions, setShowActions] = useState(false);
  // 初始化表单数据
  useEffect(() => {
    if (task) {
      // 编辑模式
      setFormData({
        name: task.name,
        completed: task.completed,
        auto: task.auto || false,
        due_to: task.due_to ? new Date(task.due_to).toISOString().slice(0, 16) : '',
        reminder: task.reminder ? new Date(task.reminder).toISOString().slice(0, 16) : '',
        actions: task.actions || []
      });
    } else {
      // 创建模式
      setFormData(initialFormData);
    }
  }, [task, isOpen]);

  const handleInputChange = (field: keyof TaskFormData, value: string | boolean | Action[]) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const handleActionChange = (index: number, field: keyof Action, value: string | number | string[]) => {
    const newActions = [...formData.actions];
    newActions[index] = { ...newActions[index], [field]: value };
    setFormData(prev => ({ ...prev, actions: newActions }));
  };

  const addAction = () => {
    setShowActions(!showActions);
    // const newAction: Action = {
    //   id: `action_${Date.now()}`,
    //   name: '',
    //   description: '',
    //   type: 'exec_command',
    //   wait: 0,
    //   cmd: '',
    //   args: []
    // };
    // setFormData(prev => ({ ...prev, actions: [...prev.actions, newAction] }));
  };

  // const removeAction = (index: number) => {
  //   setFormData(prev => ({
  //     ...prev,
  //     actions: prev.actions.filter((_, i) => i !== index)
  //   }));
  // };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const taskData: Partial<Task> = {
      name: formData.name,
      completed: formData.completed,
      auto: formData.auto,
      due_to: formData.due_to ? new Date(formData.due_to) : undefined,
      reminder: formData.reminder ? new Date(formData.reminder) : undefined,
      actions: formData.actions.length > 0 ? formData.actions : undefined
    };

    if (task) {
      taskData.id = task.id;
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
              <label htmlFor="reminder_at">提醒时间</label>
              <input
                type="datetime-local"
                id="reminder_at"
                value={formData.reminder}
                onChange={(e) => handleInputChange('reminder', e.target.value)}
              />
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
                  <div className="section-header">
                    <h3>关联动作</h3>
                    <Popover>
                      <PopoverTrigger asChild>
                        <button type="button" className="add-action-btn" onClick={addAction}>
                          <span className="material-symbols-outlined">add</span>
                          添加动作
                        </button>
                      </PopoverTrigger>
                      <PopoverContent style={{"zIndex": 1500, "width": "600px", "maxHeight": "400px", "overflowY": "auto"}}>
                        <ActionSelect />
                      </PopoverContent>
                    </Popover>

                  </div>

                  {/* {formData.actions.map((action, index) => (
                    <div key={action.id} className="action-item">
                      <div className="action-header">
                        <span>动作 {index + 1}</span>
                        <button
                          type="button"
                          className="remove-action-btn"
                          onClick={() => removeAction(index)}
                        >
                          <span className="material-symbols-outlined">delete</span>
                        </button>
                      </div>
                      
                      <div className="form-row">
                        <div className="form-group">
                          <label>动作名称</label>
                          <input
                            type="text"
                            value={action.name}
                            onChange={(e) => handleActionChange(index, 'name', e.target.value)}
                            placeholder="动作名称"
                          />
                        </div>
                        <div className="form-group">
                          <label>动作类型</label>
                          <select
                            value={action.type}
                            onChange={(e) => handleActionChange(index, 'type', e.target.value)}
                          >
                            {actionTypes.map(type => (
                              <option key={type.value} value={type.value}>
                                {type.label}
                              </option>
                            ))}
                          </select>
                        </div>
                      </div>

                      <div className="form-group">
                        <label>描述</label>
                        <input
                          type="text"
                          value={action.description}
                          onChange={(e) => handleActionChange(index, 'description', e.target.value)}
                          placeholder="动作描述"
                        />
                      </div>

                      <div className="form-row">
                        <div className="form-group">
                          <label>命令/路径</label>
                          <input
                            type="text"
                            value={action.cmd}
                            onChange={(e) => handleActionChange(index, 'cmd', e.target.value)}
                            placeholder="命令或文件路径"
                          />
                        </div>
                        <div className="form-group">
                          <label>等待时间(秒)</label>
                          <input
                            type="number"
                            value={action.wait}
                            onChange={(e) => handleActionChange(index, 'wait', parseInt(e.target.value) || 0)}
                            min="0"
                          />
                        </div>
                      </div>

                      <div className="form-group">
                        <label>参数 (用逗号分隔)</label>
                        <input
                          type="text"
                          value={action.args.join(', ')}
                          onChange={(e) => handleActionChange(index, 'args', e.target.value.split(',').map(s => s.trim()).filter(s => s))}
                          placeholder="参数1, 参数2, 参数3"
                        />
                      </div>
                    </div>
                  ))} */}
                </div>
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
    </div>
  );
}