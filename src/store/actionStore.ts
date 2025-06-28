import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type {
  ActionState,
  Action,
  CreateActionData,
  UpdateActionData,
  ActionFilters,
  ActionStats,
  ExecutionResult
} from './types';
import * as actionApi from '../api/modules/action';

// 动作状态管理接口
interface ActionStore extends ActionState {
  // CRUD 操作
  fetchActions: () => Promise<void>;
  createAction: (actionData: CreateActionData) => Promise<Action>;
  updateAction: (id: string, actionData: UpdateActionData) => Promise<Action>;
  deleteAction: (id: string) => Promise<void>;

  // 执行操作
  executeActions: (actions: Action[]) => Promise<ExecutionResult[]>;
  executeSingleAction: (action: Action) => Promise<ExecutionResult>;

  // 状态管理
  setCurrentAction: (action: Action | null) => void;
  setFilters: (filters: Partial<ActionFilters>) => void;
  clearError: () => void;
  clearExecutionResults: () => void;

  // 批量操作
  bulkUpdateActions: (ids: string[], updates: UpdateActionData) => Promise<void>;
  bulkDeleteActions: (ids: string[]) => Promise<void>;

  // 查询方法（计算属性）
  getActionById: (id: string) => Action | undefined;
  getActionsByType: (type: Action['type']) => Action[];
  getActionsByTypeGrouped: () => Record<Action['type'], Action[]>;
  getFilteredActions: () => Action[];
  getActionStats: () => ActionStats;

  // 验证方法
  validateAction: (action: Partial<Action>) => string[];

  // 工具方法
  reset: () => void;
}

// 初始状态
const initialState: ActionState = {
  actions: [],
  currentAction: null,
  loading: false,
  error: null,
  executing: false,
  executionResults: [],
  filters: {},
};

// 辅助函数：过滤动作
const filterActions = (actions: Action[], filters: ActionFilters): Action[] => {
  return actions.filter(action => {
    // 类型过滤
    if (filters.type && action.type !== filters.type) {
      return false;
    }

    // 搜索过滤
    if (filters.search) {
      const searchLower = filters.search.toLowerCase();
      return (
        action.name.toLowerCase().includes(searchLower) ||
        action.desc.toLowerCase().includes(searchLower) ||
        (action.command && action.command.toLowerCase().includes(searchLower))
      );
    }
    return true;
  });
};

// 创建动作状态store
export const useActionStore = create<ActionStore>()(
  subscribeWithSelector(
    devtools(
      (set, get) => ({
        // 初始状态
        ...initialState,

        // CRUD 操作
        fetchActions: async () => {
          set({ loading: true, error: null }, false);
          try {
            const actions = await actionApi.get_all_actions();
            set({ actions: actions, loading: false }, false);
          } catch (error) {
            set({
              loading: false,
              error: error instanceof Error ? error.message : '获取动作失败'
            }, false);
            throw error;
          }
        },

        createAction: async (actionData) => {
          // 验证动作
          const errors = get().validateAction(actionData);
          if (errors.length > 0) {
            set({ error: errors.join(', ') }, false);
            throw new Error(errors.join(', '));
          }

          set({ loading: true, error: null }, false);
          try {

            const response = await actionApi.create_action(actionData) as Action;
            const newAction = { ...response, id: response.id } as Action;
            if (!newAction) {
              throw new Error('创建动作失败：API未返回新动作');
            }

            // 重新获取动作列表以确保数据同步
            const actions = await actionApi.get_all_actions();

            set({ actions: actions || [], loading: false }, false);
            return newAction;
          } catch (error) {
            set({
              loading: false,
              error: error instanceof Error ? error.message : '创建动作失败'
            }, false);
            throw error;
          }
        },

        updateAction: async (id, actionData) => {
          // 验证动作
          const currentAction = get().getActionById(id);
          if (!currentAction) throw new Error('动作不存在');

          // 合并数据确保所有必需字段都存在
          const fullActionData = {
            name: currentAction.name,
            desc: currentAction.desc,
            type: currentAction.type,
            wait: currentAction.wait,
            command: currentAction.command,
            url: currentAction.url,
            ...actionData
          };

          const errors = get().validateAction(fullActionData);
          if (errors.length > 0) {
            set({ error: errors.join(', ') }, false, 'action/updateAction/validation');
            throw new Error(errors.join(', '));
          }

          set({ loading: true, error: null }, false, 'action/updateAction/start');
          try {
            const updatedAction = await actionApi.update_action(id, fullActionData);
            if (!updatedAction) {
              throw new Error('更新动作失败：API未返回更新后的动作');
            }

            // 重新获取动作列表以确保数据同步
            const actions = await actionApi.get_all_actions();

            set(state => ({
              actions: actions || [],
              currentAction: state.currentAction?.id === id ? updatedAction : state.currentAction,
              loading: false
            }), false, 'action/updateAction/success');

            return updatedAction;
          } catch (error) {
            set({
              loading: false,
              error: error instanceof Error ? error.message : '更新动作失败'
            }, false, 'action/updateAction/error');
            throw error;
          }
        },

        deleteAction: async (id) => {
          set({ loading: true, error: null }, false, 'action/deleteAction/start');
          try {
            await actionApi.delete_action(id);

            // 重新获取动作列表以确保数据同步
            const actions = await actionApi.get_all_actions();

            set(state => ({
              actions: actions || [],
              currentAction: state.currentAction?.id === id ? null : state.currentAction,
              loading: false
            }), false, 'action/deleteAction/success');
          } catch (error) {
            set({
              loading: false,
              error: error instanceof Error ? error.message : '删除动作失败'
            }, false, 'action/deleteAction/error');
            throw error;
          }
        },

        // 执行操作
        executeActions: async (actions) => {
          set({ executing: true, error: null }, false, 'action/executeActions/start');
          try {
            const results: ExecutionResult[] = [];

            // 顺序执行动作
            for (const action of actions) {
              const result = await get().executeSingleAction(action);
              results.push(result);

              // 如果执行失败，停止后续执行
              if (!result.success) break;
            }

            set({ executing: false }, false, 'action/executeActions/complete');
            return results;
          } catch (error) {
            set({
              executing: false,
              error: error instanceof Error ? error.message : '执行动作失败'
            }, false, 'action/executeActions/error');
            throw error;
          }
        },

        executeSingleAction: async (action) => {
          if (!action) throw new Error('动作不存在');

          set({ executing: true, error: null }, false, 'action/executeSingleAction/start');
          try {
            let result: ExecutionResult;

            // 模拟API调用延迟
            await new Promise(resolve => setTimeout(resolve, 500));

            switch (action.type) {
              case 'exec_command':
                if (!action.command) throw new Error('命令不能为空');
                // 模拟命令执行
                console.log(`模拟执行命令: ${action.command}`);
                result = {
                  actionId: action.id,
                  success: true,
                  output: `模拟执行命令: ${action.command}\n执行成功！`,
                  executedAt: new Date().toISOString(),
                };
                break;

              case 'open_url':
                if (!action.url) throw new Error('URL不能为空');
                // 模拟打开URL
                console.log(`模拟打开URL: ${action.url}`);
                // 如果在浏览器环境中，可以尝试实际打开URL
                if (typeof window !== 'undefined') {
                  try {
                    window.open(action.url, '_blank');
                  } catch (e) {
                    console.error('无法打开URL:', e);
                  }
                }
                result = {
                  actionId: action.id,
                  success: true,
                  executedAt: new Date().toISOString(),
                };
                break;

              default:
                throw new Error(`不支持的动作类型: ${action.type}`);
            }

            // 添加执行结果
            set(state => ({
              executionResults: [result, ...state.executionResults],
              executing: false
            }), false, 'action/executeSingleAction/success');

            return result;
          } catch (error) {
            const errorResult: ExecutionResult = {
              actionId: action.id,
              success: false,
              error: error instanceof Error ? error.message : '执行失败',
              executedAt: new Date().toISOString(),
            };

            set(state => ({
              executionResults: [errorResult, ...state.executionResults],
              executing: false,
              error: errorResult.error
            }), false, 'action/executeSingleAction/error');

            return errorResult;
          }
        },

        // 状态管理
        setCurrentAction: (action) => {
          set({ currentAction: action }, false, 'action/setCurrentAction');
        },

        setFilters: (filters) => {
          set(state => ({
            filters: { ...state.filters, ...filters }
          }), false, 'action/setFilters');
        },

        clearError: () => {
          set({ error: null }, false, 'action/clearError');
        },

        clearExecutionResults: () => {
          set({ executionResults: [] }, false, 'action/clearExecutionResults');
        },

        // 批量操作
        bulkUpdateActions: async (ids, updates) => {
          set({ loading: true, error: null }, false, 'action/bulkUpdate/start');
          try {
            const promises = ids.map(id => {
              const currentAction = get().getActionById(id);
              if (!currentAction) {
                throw new Error(`动作 ${id} 不存在`);
              }

              // 合并数据确保所有必需字段都存在
              const fullActionData = {
                name: currentAction.name,
                desc: currentAction.desc,
                type: currentAction.type,
                wait: currentAction.wait,
                command: currentAction.command,
                url: currentAction.url,
                ...updates
              };

              return actionApi.update_action(id, fullActionData);
            });
            await Promise.all(promises);

            // 重新获取动作列表以确保数据同步
            const actions = await actionApi.get_all_actions();

            set({ actions: actions || [], loading: false }, false, 'action/bulkUpdate/success');
          } catch (error) {
            set({
              loading: false,
              error: error instanceof Error ? error.message : '批量更新失败'
            }, false, 'action/bulkUpdate/error');
            throw error;
          }
        },

        bulkDeleteActions: async (ids) => {
          set({ loading: true, error: null }, false, 'action/bulkDelete/start');
          try {
            const promises = ids.map(id => actionApi.delete_action(id));
            await Promise.all(promises);

            // 重新获取动作列表以确保数据同步
            const actions = await actionApi.get_all_actions();

            set(state => ({
              actions: actions || [],
              currentAction: ids.includes(state.currentAction?.id || '') ? null : state.currentAction,
              loading: false
            }), false, 'action/bulkDelete/success');
          } catch (error) {
            set({
              loading: false,
              error: error instanceof Error ? error.message : '批量删除失败'
            }, false, 'action/bulkDelete/error');
            throw error;
          }
        },

        // 查询方法（计算属性）
        getActionById: (id) => {
          return get().actions.find(action => action.id === id);
        },

        getActionsByType: (type) => {
          return get().actions.filter(action => action.type === type);
        },

        getActionsByTypeGrouped: () => {
          const result: Record<Action['type'], Action[]> = {
            'exec_command': [],
            'open_url': [],
            'open_file': [],
            'open_dir': [],
          };

          get().actions.forEach(action => {
            result[action.type].push(action);
          });

          return result;
        },

        getFilteredActions: () => {
          const { actions, filters } = get();
          return filterActions(actions, filters);
        },

        getActionStats: () => {
          const actions = get().actions;
          const byType: Record<Action['type'], number> = {
            'exec_command': 0,
            'open_url': 0,
            'open_file': 0,
            'open_dir': 0,
          };

          actions.forEach(action => {
            byType[action.type]++;
          });

          return {
            total: actions.length,
            byType,
          };
        },

        // 验证方法
        validateAction: (action) => {
          const errors: string[] = [];

          if (!action.name || action.name.trim() === '') {
            errors.push('动作名称不能为空');
          }

          if (!action.type) {
            errors.push('动作类型不能为空');
          }

          switch (action.type) {
            case 'exec_command':
              if (!action.command || action.command.trim() === '') {
                errors.push('命令不能为空');
              }
              break;

            case 'open_url':
              if (!action.url || action.url.trim() === '') {
                errors.push('URL不能为空');
              } else {
                try {
                  new URL(action.url);
                } catch (e) {
                  errors.push('URL格式不正确', e as string);
                }
              }
              break;
          }

          return errors;
        },

        // 工具方法
        reset: () => {
          set(initialState, false, 'action/reset');
        },
      }),
      {
        name: 'action-store',
      }
    )
  )
);