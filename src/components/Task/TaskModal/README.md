# TaskModal 任务创建/编辑浮动窗口组件

一个功能完整的任务创建和编辑浮动窗口组件，类似滴答清单的任务编辑界面。

## 功能特性

### 🎯 核心功能
- ✅ 创建新任务
- ✅ 编辑现有任务
- ✅ 创建子任务
- ✅ 设置任务截止时间和提醒时间
- ✅ 配置任务自动执行
- ✅ 关联多个动作(Actions)

### 🎨 界面特性
- ✅ 现代化的浮动窗口设计
- ✅ 响应式布局，支持移动端
- ✅ 平滑的动画效果
- ✅ 直观的用户交互
- ✅ 高级设置折叠面板

### 🔧 技术特性
- ✅ TypeScript 类型安全
- ✅ 表单验证
- ✅ 键盘快捷键支持
- ✅ 无障碍访问支持

## 使用方法

### 基本用法

```tsx
import TaskModal from '@/components/Task/TaskModal';
import type { Task } from '@/types';

function MyComponent() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingTask, setEditingTask] = useState<Task | null>(null);

  const handleSaveTask = (taskData: Partial<Task>) => {
    // 处理任务保存逻辑
    console.log('保存任务:', taskData);
  };

  return (
    <>
      <button onClick={() => setIsModalOpen(true)}>
        创建任务
      </button>
      
      <TaskModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onSave={handleSaveTask}
        task={editingTask} // 编辑模式时传入
      />
    </>
  );
}
```

### 创建子任务

```tsx
const handleCreateSubTask = (parentTask: Task) => {
  setParentTask(parentTask);
  setEditingTask(null);
  setIsModalOpen(true);
};

<TaskModal
  isOpen={isModalOpen}
  onClose={() => setIsModalOpen(false)}
  onSave={handleSaveTask}
  parentTask={parentTask} // 传入父任务
/>
```

### 编辑现有任务

```tsx
const handleEditTask = (task: Task) => {
  setEditingTask(task);
  setIsModalOpen(true);
};

<TaskModal
  isOpen={isModalOpen}
  onClose={() => setIsModalOpen(false)}
  onSave={handleSaveTask}
  task={editingTask} // 传入要编辑的任务
/>
```

## Props 接口

```tsx
interface TaskModalProps {
  isOpen: boolean;              // 控制模态框显示/隐藏
  onClose: () => void;          // 关闭模态框的回调
  onSave: (task: Partial<Task>) => void; // 保存任务的回调
  task?: Task | null;           // 编辑模式时的任务数据
  parentTask?: Task | null;     // 创建子任务时的父任务
}
```

## 表单字段

### 基本信息
- **任务标题** (必填): 任务的名称
- **截止时间**: 任务的截止日期和时间
- **提醒时间**: 任务的提醒日期和时间
- **已完成**: 任务完成状态
- **自动执行**: 是否自动执行关联的动作

### 高级设置
- **关联动作**: 可以为任务添加多个动作
  - 动作名称
  - 动作类型 (执行命令/打开文件/打开目录/打开链接)
  - 动作描述
  - 命令或路径
  - 等待时间
  - 参数列表

## 动作类型

| 类型 | 值 | 描述 |
|------|----|---------|
| 执行命令 | `exec_command` | 执行系统命令 |
| 打开文件 | `open_file` | 打开指定文件 |
| 打开目录 | `open_dir` | 打开指定目录 |
| 打开链接 | `open_url` | 打开网页链接 |

## 样式定制

组件使用CSS变量，可以通过覆盖CSS变量来定制样式：

```css
.task-modal {
  --modal-bg: white;
  --modal-border-radius: 12px;
  --primary-color: #3b82f6;
  --text-color: #1f2937;
  --border-color: #e5e7eb;
}
```

## 键盘快捷键

- `Esc`: 关闭模态框
- `Ctrl/Cmd + Enter`: 保存任务
- `Tab`: 在表单字段间导航

## 响应式设计

组件在不同屏幕尺寸下自动调整布局：

- **桌面端**: 双列布局，完整功能
- **平板端**: 单列布局，保持功能完整性
- **移动端**: 优化的单列布局，按钮全宽显示

## 无障碍访问

- 支持键盘导航
- 适当的ARIA标签
- 语义化的HTML结构
- 高对比度支持

## 示例

查看 `example.tsx` 文件获取完整的使用示例，包括：
- 任务列表管理
- 创建、编辑、删除任务
- 子任务管理
- 状态更新

## 注意事项

1. 确保项目中已安装并配置了 Material Icons
2. 组件依赖 `@/types` 中的 `Task` 和 `Action` 类型定义
3. 建议在使用前先查看示例代码了解完整的集成方式
4. 表单验证要求任务标题不能为空
5. 时间字段使用 `datetime-local` 输入类型，确保浏览器兼容性