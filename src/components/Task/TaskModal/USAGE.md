# TaskModal 组件使用指南

## 快速开始

### 1. 导入组件

```tsx
import { TaskModal } from '@/components/Task';
// 或者
import TaskModal from '@/components/Task/TaskModal';
```

### 2. 基本使用

```tsx
import React, { useState } from 'react';
import { TaskModal } from '@/components/Task';
import type { Task } from '@/types';

function App() {
  const [isOpen, setIsOpen] = useState(false);
  
  const handleSave = (taskData: Partial<Task>) => {
    console.log('保存任务:', taskData);
    // 在这里处理任务保存逻辑
  };
  
  return (
    <div>
      <button onClick={() => setIsOpen(true)}>
        创建任务
      </button>
      
      <TaskModal
        isOpen={isOpen}
        onClose={() => setIsOpen(false)}
        onSave={handleSave}
      />
    </div>
  );
}
```

## 使用场景

### 场景1: 创建新任务

```tsx
const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);

<TaskModal
  isOpen={isCreateModalOpen}
  onClose={() => setIsCreateModalOpen(false)}
  onSave={handleCreateTask}
/>
```

### 场景2: 编辑现有任务

```tsx
const [editingTask, setEditingTask] = useState<Task | null>(null);
const [isEditModalOpen, setIsEditModalOpen] = useState(false);

const handleEditTask = (task: Task) => {
  setEditingTask(task);
  setIsEditModalOpen(true);
};

<TaskModal
  isOpen={isEditModalOpen}
  onClose={() => {
    setIsEditModalOpen(false);
    setEditingTask(null);
  }}
  onSave={handleUpdateTask}
  task={editingTask}
/>
```

### 场景3: 创建子任务

```tsx
const [parentTask, setParentTask] = useState<Task | null>(null);
const [isSubTaskModalOpen, setIsSubTaskModalOpen] = useState(false);

const handleCreateSubTask = (parent: Task) => {
  setParentTask(parent);
  setIsSubTaskModalOpen(true);
};

<TaskModal
  isOpen={isSubTaskModalOpen}
  onClose={() => {
    setIsSubTaskModalOpen(false);
    setParentTask(null);
  }}
  onSave={handleCreateSubTask}
  parentTask={parentTask}
/>
```

## 完整示例

```tsx
import React, { useState } from 'react';
import { TaskModal } from '@/components/Task';
import type { Task } from '@/types';

function TaskManager() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingTask, setEditingTask] = useState<Task | null>(null);
  const [parentTask, setParentTask] = useState<Task | null>(null);

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
      // 更新现有任务
      setTasks(prev => prev.map(task => 
        task.id === editingTask.id 
          ? { ...task, ...taskData }
          : task
      ));
    } else {
      // 创建新任务
      const newTask: Task = {
        id: Date.now(),
        create_at: new Date(),
        completed: false,
        auto: false,
        ...taskData
      } as Task;

      if (parentTask) {
        // 添加为子任务
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
        // 添加为根任务
        setTasks(prev => [...prev, newTask]);
      }
    }
  };

  const handleCloseModal = () => {
    setIsModalOpen(false);
    setEditingTask(null);
    setParentTask(null);
  };

  return (
    <div>
      <button onClick={handleCreateTask}>
        创建任务
      </button>
      
      {/* 任务列表 */}
      <div>
        {tasks.map(task => (
          <div key={task.id}>
            <span>{task.title}</span>
            <button onClick={() => handleEditTask(task)}>
              编辑
            </button>
            <button onClick={() => handleCreateSubTask(task)}>
              添加子任务
            </button>
          </div>
        ))}
      </div>
      
      <TaskModal
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        onSave={handleSaveTask}
        task={editingTask}
        parentTask={parentTask}
      />
    </div>
  );
}

export default TaskManager;
```

## 注意事项

1. **必需依赖**: 确保项目中已配置 Material Icons
2. **类型定义**: 确保 `@/types` 中有正确的 `Task` 和 `Action` 类型定义
3. **表单验证**: 任务标题为必填字段
4. **时间格式**: 使用 `datetime-local` 输入类型
5. **响应式**: 组件在移动端会自动调整布局

## 键盘快捷键

- `Esc`: 关闭模态框
- `Ctrl/Cmd + Enter`: 保存任务（需要自定义实现）
- `Tab`: 在表单字段间导航

## 自定义样式

可以通过覆盖 CSS 类来自定义样式：

```css
.task-modal {
  /* 自定义模态框样式 */
}

.task-modal-header {
  /* 自定义头部样式 */
}

.task-modal-form {
  /* 自定义表单样式 */
}
```

## 访问演示

访问 `/task-demo` 路由查看完整的功能演示。