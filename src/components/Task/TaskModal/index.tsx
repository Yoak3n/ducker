import React, { useState, useEffect } from 'react';
import type { Task, Action } from '@/types';
import './index.css';
import ActionSelect from '@/components/Action/ActionSelect';

// import { Button } from '@/components/ui/button';

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

// const actionTypes = [
//   { value: 'exec_command', label: '执行命令' },
//   { value: 'open_file', label: '打开文件' },
//   { value: 'open_dir', label: '打开目录' },
//   { value: 'open_url', label: '打开链接' }
// ];

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

  // const handleActionChange = (index: number, field: keyof Action, value: string | number | string[]) => {
  //   const newActions = [...formData.actions];
  //   newActions[index] = { ...newActions[index], [field]: value };
  //   setFormData(prev => ({ ...prev, actions: newActions }));
  // };

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