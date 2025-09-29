export interface Action {
    id: string;
    name: string;
    desc: string;
    wait: number;
    type: ActionTypeValue;
    retry?: number;
    timeout?: number;
    command?: string;
    args?: string[];
}

// Action数据操作类型 - 使用工具类型优化
export type CreateActionData = Omit<Action, 'id'>
export type UpdateActionData = Partial<CreateActionData>
export type ActionFormData = Omit<Action, 'id'>

// Action字段选择器类型
export type ActionBasicInfo = Pick<Action, 'id' | 'name' | 'type'>
export type ActionExecutionInfo = Pick<Action, 'wait' | 'retry' | 'timeout'>
export type ActionCommandInfo = Pick<Action, 'command' | 'args'>
export type ActionRequiredFields = Required<Pick<Action, 'name' | 'desc' | 'wait' | 'type'>>
export type ActionOptionalFields = Pick<Action, 'retry' | 'timeout' | 'command' | 'args'>

// ActionType值的联合类型 - 作为单一真实来源
export type ActionTypeValue = 'command' | 'file' | 'url' | 'notice' | 'directory' | 'group'
export type ActionTypeSelector = ActionTypeValue | 'all' 
// ActionType接口 - 使用ActionTypeValue确保类型一致性
export interface ActionType {
    value: ActionTypeSelector;
    label: string;
    icon?: string;
}

// ActionType相关工具类型
export type ActionTypeMap = Record<ActionTypeValue, string>
export type ActionTypeKeys = keyof ActionTypeMap
export type ActionsByType<T extends ActionTypeValue> = Action & { type: T }

// 类型安全的actionTypes数组 - 使用as const确保类型推断
export const actionTypes = [
  { value: 'all', label: '全部类型', icon: 'apps' },
  { value: 'file', label: '文件操作', icon: 'folder' },
  { value: 'directory', label: '目录操作', icon: 'folder_open' },
  { value: 'command', label: '命令执行', icon: 'terminal' },
  { value: 'url', label: '网页链接', icon: 'link' },
  { value: 'notice', label: '通知', icon: 'notifications' },
  { value: 'group', label: '任务组', icon: 'view_list' },
] as const satisfies readonly ActionType[]

// 从actionTypes数组提取类型，确保运行时和类型时的一致性
export type ActionTypeFromArray = typeof actionTypes[number]['value']

// ==================== Action执行相关类型 ====================

// Action执行状态
export type ActionStatus = 'pending' | 'running' | 'completed' | 'failed' | 'cancelled'

// Action执行结果
export interface ActionResult {
    actionId: string
    status: ActionStatus
    startTime: Date
    endTime?: Date
    duration?: number
    output?: string
    error?: string
    exitCode?: number
}

// Action执行上下文
export interface ActionContext {
    variables: Record<string, any>
    environment: Record<string, string>
    workingDirectory?: string
}

// Action执行配置
export interface ActionExecutionConfig {
    dryRun?: boolean
    verbose?: boolean
    continueOnError?: boolean
    maxRetries?: number
    retryDelay?: number
}

// ==================== Action批量操作类型 ====================

// Action ID集合
export type ActionIds = string[]

// 批量Action操作
export interface BulkActionOperation {
    actionIds: ActionIds
    operation: 'execute' | 'delete' | 'duplicate' | 'export'
    config?: ActionExecutionConfig
}

// 批量Action更新
export type BulkActionUpdate = {
    actionIds: ActionIds
    updates: Partial<UpdateActionData>
}

// ==================== Action查询和过滤类型 ====================

// Action过滤器
export interface ActionFilter {
    type?: ActionTypeSelector | ActionTypeSelector[]
    name?: string
    status?: ActionStatus | ActionStatus[]
    hasCommand?: boolean
    hasTimeout?: boolean
    createdAfter?: Date
    createdBefore?: Date
}

// Action排序字段
export type ActionSortField = 'name' | 'type' | 'wait' | 'timeout' | 'createdAt'

// Action排序方向
export type ActionSortOrder = 'asc' | 'desc'

// Action排序配置
export interface ActionSort {
    field: ActionSortField
    order: ActionSortOrder
}

// Action查询参数
export interface ActionQuery {
    filter?: ActionFilter
    sort?: ActionSort
    limit?: number
    offset?: number
}

// ==================== Action统计和分析类型 ====================

// Action统计信息
export interface ActionStats {
    total: number
    byType: Record<ActionTypeValue, number>
    byStatus: Record<ActionStatus, number>
    averageExecutionTime: number
    successRate: number
}

// Action性能指标
export interface ActionPerformanceMetrics {
    actionId: string
    executionCount: number
    averageDuration: number
    successCount: number
    failureCount: number
    lastExecuted?: Date
}

// ==================== Action验证和校验类型 ====================

// Action验证错误
export interface ActionValidationError {
    field: keyof Action
    message: string
    code: string
}

// Action验证结果
export interface ActionValidationResult {
    isValid: boolean
    errors: ActionValidationError[]
    warnings?: ActionValidationError[]
}

// Action依赖关系
export interface ActionDependency {
    actionId: string
    dependsOn: string[]
    dependents: string[]
}

// ==================== Action扩展类型 ====================

// 带有执行状态的Action
export type ActionWithStatus = Action & {
    status: ActionStatus
    lastResult?: ActionResult
}

// 带有统计信息的Action
export type ActionWithStats = Action & {
    stats: ActionPerformanceMetrics
}

// Action模板
export type ActionTemplate = Omit<Action, 'id'> & {
    templateId: string
    category: string
    description: string
    variables?: string[]
}