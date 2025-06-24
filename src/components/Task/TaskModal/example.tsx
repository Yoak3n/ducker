import React, { useState } from 'react';
import TaskModal from './index';
import type { Task } from '@/types';

// 使用示例组件
export default function TaskModalExample() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingTask, setEditingTask] = useState<Task | null>(null);
  const [parentTask, setParentTask] = useState<Task | null>(null);
  const [tasks, setTasks] = useState<Task[]>([
    {
      id: 1,
      title: '示例任务',
      completed: false,
      auto: false,
      create_at: new Date(),
      due_at: new Date(Date.now() + 24 * 60 * 60 * 1000), // 明天
      actions: [
        {
          id: 'action1',
          name: '打开项目',
          description: '打开项目文件夹',
          type: 'open_dir',
          wait: 0,
          cmd: '/path/to/project',
          args: []
        }
      ]
    }
  ]);

  // 创建新任务
  const handleCreateTask = () => {
    setEditingTask(null);
    setParentTask(null);
    setIsModalOpen(true);
  };

  // 编辑任务
  const handleEditTask = (task: Task) => {
    setEditingTask(task);
    setParentTask(null);
    setIsModalOpen(true);
  };

  // 创建子任务
  const handleCreateSubTask = (parent: Task) => {
    setEditingTask(null);
    setParentTask(parent);
    setIsModalOpen(true);
  };

  // 保存任务
  const handleSaveTask = (taskData: Partial<Task>) => {
    if (editingTask) {
      // 编辑模式
      setTasks(prev => prev.map(task => 
        task.id === editingTask.id 
          ? { ...task, ...taskData }
          : task
      ));
    } else {
      // 创建模式
      const newTask: Task = {
        id: Date.now(),
        create_at: new Date(),
        completed: false,
        auto: false,
        ...taskData
      } as Task;

      if (parentTask) {
        // 创建子任务
        setTasks(prev => prev.map(task => {
          if (task.id === parentTask.id) {
            return {
              ...task,
              children: [...(task.children || []), newTask]
            };
          }
          return task;
        }));
      } else {
        // 创建根任务
        setTasks(prev => [...prev, newTask]);
      }
    }
  };

  return (
    <div style={{ padding: '20px' }}>
      <h2>任务管理示例</h2>
      
      <div style={{ marginBottom: '20px' }}>
        <button 
          onClick={handleCreateTask}
          style={{
            padding: '10px 20px',
            backgroundColor: '#3b82f6',
            color: 'white',
            border: 'none',
            borderRadius: '6px',
            cursor: 'pointer',
            marginRight: '10px'
          }}
        >
          创建任务
        </button>
      </div>

      <div>
        <h3>任务列表</h3>
        {tasks.map(task => (
          <div 
            key={task.id} 
            style={{
              border: '1px solid #e5e7eb',
              borderRadius: '8px',
              padding: '16px',
              marginBottom: '12px',
              backgroundColor: task.completed ? '#f0f9ff' : 'white'
            }}
          >
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h4 style={{ margin: '0 0 8px 0', color: task.completed ? '#6b7280' : '#1f2937' }}>
                  {task.title}
                </h4>
                <div style={{ fontSize: '0.875rem', color: '#6b7280' }}>
                  状态: {task.completed ? '已完成' : '进行中'} | 
                  自动执行: {task.auto ? '是' : '否'} |
                  动作数量: {task.actions?.length || 0}
                </div>
                {task.due_at && (
                  <div style={{ fontSize: '0.875rem', color: '#ef4444', marginTop: '4px' }}>
                    截止时间: {new Date(task.due_at).toLocaleString()}
                  </div>
                )}
              </div>
              <div>
                <button 
                  onClick={() => handleEditTask(task)}
                  style={{
                    padding: '6px 12px',
                    backgroundColor: '#f3f4f6',
                    color: '#374151',
                    border: '1px solid #d1d5db',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    marginRight: '8px',
                    fontSize: '0.875rem'
                  }}
                >
                  编辑
                </button>
                <button 
                  onClick={() => handleCreateSubTask(task)}
                  style={{
                    padding: '6px 12px',
                    backgroundColor: '#10b981',
                    color: 'white',
                    border: 'none',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    fontSize: '0.875rem'
                  }}
                >
                  添加子任务
                </button>
              </div>
            </div>
            
            {/* 显示子任务 */}
            {task.children && task.children.length > 0 && (
              <div style={{ marginTop: '12px', paddingLeft: '20px' }}>
                <h5 style={{ margin: '0 0 8px 0', color: '#6b7280' }}>子任务:</h5>
                {task.children.map(child => (
                  <div 
                    key={child.id}
                    style={{
                      padding: '8px 12px',
                      backgroundColor: '#f9fafb',
                      border: '1px solid #e5e7eb',
                      borderRadius: '4px',
                      marginBottom: '4px',
                      fontSize: '0.875rem'
                    }}
                  >
                    {child.title} - {child.completed ? '已完成' : '进行中'}
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>

      <TaskModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onSave={handleSaveTask}
        task={editingTask}
        parentTask={parentTask}
      />
    </div>
  );
}