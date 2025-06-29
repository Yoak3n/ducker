import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { TaskState, Task, TaskData, TaskFilters, TaskStats } from './types';
import * as taskApi from '@/api/modules/task';

// 任务状态管理接口
interface TaskStore extends TaskState {
  // CRUD 操作
  fetchTasks: () => Promise<void>;
  createTask: (taskData: TaskData) => Promise<Task>;
  updateTask: (id: string, taskData: TaskData) => Promise<Task>;
  deleteTask: (id: string) => Promise<void>;
  
  // 状态管理
  setCurrentTask: (task: Task | null) => void;
  setFilters: (filters: Partial<TaskFilters>) => void;
  clearError: () => void;
  
  // 便捷操作
  toggleTaskCompletion: (id: string) => Promise<void>;
  bulkUpdateTasks: (ids: string[], updates: TaskData) => Promise<void>;
  bulkDeleteTasks: (ids: string[]) => Promise<void>;
  
  // 查询方法（计算属性）
  getTaskById: (id: string) => Task | undefined;
  getCompletedTasks: () => Task[];
  getPendingTasks: () => Task[];
  getFilteredTasks: () => Task[];
  getTasksByDateRange: (start: Date, end: Date) => Task[];
  getTaskStats: () => TaskStats;
  
  // 工具方法
  reset: () => void;
}

// 初始状态
const initialState: TaskState = {
  tasks: [],
  currentTask: null,
  loading: false,
  error: null,
  filters: {},
};

// 辅助函数：过滤任务
const filterTasks = (tasks: Task[], filters: TaskFilters): Task[] => {
  return tasks.filter(task => {
    // 完成状态过滤
    if (filters.completed !== undefined && task.completed !== filters.completed) {
      return false;
    }
    
    // 搜索过滤
    if (filters.search && !task.name.toLowerCase().includes(filters.search.toLowerCase())) {
      return false;
    }
    
    // 日期范围过滤
    if (filters.dateRange && task.created_at) {
      const taskDate = new Date(task.created_at);
      if (taskDate < filters.dateRange.start || taskDate > filters.dateRange.end) {
        return false;
      }
    }
    
    return true;
  });
};

// 创建任务状态store
export const useTaskStore = create<TaskStore>()(
  subscribeWithSelector(
    devtools(
      (set, get) => ({
        // 初始状态
        ...initialState,
        
        // CRUD 操作
        fetchTasks: async () => {
          set({ loading: true, error: null }, false);
          try {
            const tasks = await taskApi.get_all_tasks();
            set({ tasks: tasks || [], loading: false }, false);
          } catch (error) {
            set({ 
              loading: false, 
              error: error instanceof Error ? error.message : '获取任务失败' 
            }, false);
          }
        },
        
        createTask: async (taskData) => {
          set({ loading: true, error: null }, false);
          try {
            const taskId = await taskApi.create_task(taskData);
            if (!taskId) {
              throw new Error('创建任务失败：未返回任务ID');
            }
            
            // 重新获取任务列表以确保数据同步
            const tasks = await taskApi.get_all_tasks();
            const newTask = tasks?.find(task => task.id === taskId);
            
            if (!newTask) {
              throw new Error('创建任务失败：无法找到新创建的任务');
            }
            
            set({ tasks: tasks || [], loading: false }, false, 'task/createTask/success');
            return newTask;
          } catch (error) {
            set({ 
              loading: false, 
              error: error instanceof Error ? error.message : '创建任务失败' 
            }, false, 'task/createTask/error');
            throw error;
          }
        },
        
        updateTask: async (id, taskData) => {
          set({ loading: true, error: null }, false);
          try {
            // 获取当前任务以合并完整数据
            const currentTask = get().getTaskById(id);
            if (!currentTask) {
              throw new Error('任务不存在');
            }
            
            // 合并数据确保所有必需字段都存在
            const fullTaskData = {
              value: currentTask.value,
              ...taskData
            };
            
            const updatedTask = await taskApi.update_task(id, fullTaskData);
            if (!updatedTask) {
              throw new Error('更新任务失败：API未返回更新后的任务');
            }
            
            // 重新获取任务列表以确保数据同步
            const tasks = await taskApi.get_all_tasks();
            
            set(state => ({
              tasks: tasks || [],
              currentTask: state.currentTask?.id === id ? updatedTask : state.currentTask,
              loading: false
            }), false);
            
            return updatedTask;
          } catch (error) {
            set({ 
              loading: false, 
              error: error instanceof Error ? error.message : '更新任务失败' 
            }, false);
            throw error;
          }
        },
        
        deleteTask: async (id) => {
          set({ loading: true, error: null }, false);
          try {
            await taskApi.delete_task(id);
            
            // 重新获取任务列表以确保数据同步
            const tasks = await taskApi.get_all_tasks();
            
            set(state => ({
              tasks: tasks || [],
              currentTask: state.currentTask?.id === id ? null : state.currentTask,
              loading: false
            }), false);
          } catch (error) {
            set({ 
              loading: false, 
              error: error instanceof Error ? error.message : '删除任务失败' 
            }, false);
            throw error;
          }
        },
        
        // 状态管理
        setCurrentTask: (task) => {
          set({ currentTask: task }, false);
        },
        
        setFilters: (filters) => {
          set(state => ({ 
            filters: { ...state.filters, ...filters } 
          }), false);
        },
        
        clearError: () => {
          set({ error: null }, false);
        },
        
        // 便捷操作
        toggleTaskCompletion: async (id) => {
          const task = get().getTaskById(id);
          if (!task) throw new Error('任务不存在');
          // TODO新建任务时不应该有actions
          await get().updateTask(id, {...task, completed: !task.completed,actions:[] });
        },
        
        bulkUpdateTasks: async (ids, updates) => {
          set({ loading: true, error: null }, false);
          try {
            const promises = ids.map(id => {
              const currentTask = get().getTaskById(id);
              if (!currentTask) {
                throw new Error(`任务 ${id} 不存在`);
              }
              
              // 合并数据确保所有必需字段都存在
              const fullTaskData = {
                value: currentTask.value,
                ...updates
              };
              
              return taskApi.update_task(id, fullTaskData);
            });
            await Promise.all(promises);
            
            // 重新获取任务列表以确保数据同步
            const tasks = await taskApi.get_all_tasks();
            
            set({ tasks: tasks || [], loading: false }, false, 'task/bulkUpdate/success');
          } catch (error) {
            set({ 
              loading: false, 
              error: error instanceof Error ? error.message : '批量更新失败' 
            }, false, 'task/bulkUpdate/error');
            throw error;
          }
        },
        
        bulkDeleteTasks: async (ids) => {
          set({ loading: true, error: null }, false, 'task/bulkDelete/start');
          try {
            const promises = ids.map(id => taskApi.delete_task(id));
            await Promise.all(promises);
            
            // 重新获取任务列表以确保数据同步
            const tasks = await taskApi.get_all_tasks();
            
            set(state => ({
              tasks: tasks || [],
              currentTask: ids.includes(state.currentTask?.id || '') ? null : state.currentTask,
              loading: false
            }), false, 'task/bulkDelete/success');
          } catch (error) {
            set({ 
              loading: false, 
              error: error instanceof Error ? error.message : '批量删除失败' 
            }, false, 'task/bulkDelete/error');
            throw error;
          }
        },
        
        // 查询方法（计算属性）
        getTaskById: (id) => {
          return get().tasks.find(task => task.id === id);
        },
        
        getCompletedTasks: () => {
          return get().tasks.filter(task => task.completed);
        },
        
        getPendingTasks: () => {
          return get().tasks.filter(task => !task.completed);
        },
        
        getFilteredTasks: () => {
          const { tasks, filters } = get();
          return filterTasks(tasks, filters);
        },
        
        getTasksByDateRange: (start, end) => {
          return get().tasks.filter(task => {
            if (!task.created_at) return false;
            const taskDate = new Date(task.created_at);
            return taskDate >= start && taskDate <= end;
          });
        },
        
        getTaskStats: () => {
          const tasks = get().tasks;
          const total = tasks.length;
          const completed = tasks.filter(task => task.completed).length;
          const pending = total - completed;
          const completionRate = total > 0 ? (completed / total) * 100 : 0;
          
          return {
            total,
            completed,
            pending,
            completionRate,
          };
        },
        
        // 工具方法
        reset: () => {
          set(initialState, false, 'task/reset');
        },
      }),
      {
        name: 'task-store',
      }
    )
  )
);