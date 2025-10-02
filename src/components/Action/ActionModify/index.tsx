import React, { useState, useCallback, useMemo, useEffect } from 'react';
import { useParams,useNavigate } from 'react-router-dom';
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"

import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Card, CardAction, CardContent, CardHeader, CardTitle, } from "@/components/ui/card"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue, } from "@/components/ui/select"
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { toast } from "sonner"

import { useActionStore } from '@/store';
import { open_file_select_dialog } from '@/utils';
import type { Action, ActionTypeValue } from '@/types';
import ActionSelect from '@/components/Action/ActionSelect';

const formSchema = z.object({
    name: z.string().min(2, {
        message: "Action name must be at least 2 characters.",
    }).max(50, {
        message: "Action name must be less than 50 characters.",
    }),
    type: z.enum(["file", "directory", "command", "url", "group", "notice"]),
    command: z.string().optional(),
    args: z.string().optional(),
    desc: z.string().max(200, {
        message: "Description must be less than 200 characters.",
    }).optional(),
    wait: z.coerce.number().min(0, "Wait time must be non-negative"),
    retry: z.coerce.number().min(0, "Retry count must be non-negative").max(10, "Retry count must be less than 10").optional(),
    timeout: z.coerce.number().min(0, "Timeout must be non-negative").optional(),
}).refine((data) => {
    // 非Group类型必须有command
    if (data.type !== "group" && (!data.command || data.command.trim() === "")) {
        return false;
    }
    return true;
}, {
    message: "Command/Path/URL/Title is required",
    path: ["command"],
}).refine((data) => {
    // URL类型的特殊验证
    if (data.type === "url") {
        try {
            new URL(data.command!);
            return true;
        } catch {
            return false;
        }
    }
    return true;
}, {
    message: "Please enter a valid URL",
    path: ["command"],
}).refine((data) => {
    // Group类型必须有action IDs
    if (data.type === "group") {
        if (!data.args || data.args.trim() === "") {
            return false;
        }
        // 验证args是否包含有效的action IDs
        const actionIds = data.args.split(',').filter(id => id.trim());
        return actionIds.length > 0;
    }
    return true;
}, {
    message: "请至少选择一个Action",
    path: ["args"],
})

const ActionModify: React.FC = () => {
    const [currentActionType, setCurrentActionType] = useState<ActionTypeValue>("file");
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [isActionSelectOpen, setIsActionSelectOpen] = useState(false);
    const [selectedActions, setSelectedActions] = useState<Action[]>([]);

    const navigate = useNavigate();
    const params = useParams()
    const id = params.id;

    // 使用选择器来避免无限循环
    const actions = useActionStore(state => state.actions);
    const fetchActions = useActionStore(state => state.fetchActions);
    const createAction = useActionStore(state => state.createAction);
    const updateAction = useActionStore(state => state.updateAction);

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            type: "file",
            command: "",
            args: "",
            desc: "",
            wait: 0,
            retry: undefined,
            timeout: undefined,
        },
    })

    const handleTypeChange = useCallback((type: ActionTypeValue, preserveFields = false) => {
        setCurrentActionType(type);
        // 只在非保留字段模式下清空command和args字段，避免不同类型间的数据混淆
        if (!preserveFields) {
            form.setValue('command', '');
            form.setValue('args', '');
            // 清空选中的actions
            setSelectedActions([]);
        }
    }, []);

    const currentAction = useMemo(() => {
        // 当 id 存在时，查找对应的 action（编辑模式）
        if (id) {
            return actions.find(action => action.id === id) || null;
        }
        return null;
    }, [id, actions]);
    
    const isAddAction = useMemo(() => {
        // 当没有 id 参数时为创建模式
        return !id;
    }, [id]);

    // 确保在组件挂载时获取 actions 数据
    useEffect(() => {
        if (actions.length === 0) {
            fetchActions();
        }
    }, []);

    // 处理表单数据填入
    useEffect(() => {
        if (!currentAction) {
            // 新增模式：设置默认值
            const defaultValues = {
                name: "",
                type: "file" as ActionTypeValue,
                command: "",
                args: "",
                desc: "",
                wait: 0,
                retry: undefined,
                timeout: undefined,
            };
            form.reset(defaultValues);
            setCurrentActionType("file");
            setSelectedActions([]);
        } else if (currentAction) {
            const formData = {
                name: currentAction.name,
                type: currentAction.type,
                command: currentAction.command || "",
                args: currentAction.args?.join(",") || "",
                desc: currentAction.desc || "",
                wait: currentAction.wait,
                retry: currentAction.retry,
                timeout: currentAction.timeout,
            };
            form.reset(formData);
            setCurrentActionType(currentAction.type);
            
            // 如果是 group 类型，设置选中的 actions
            if (currentAction.type === 'group' && currentAction.args) {
                const actionIds = currentAction.args;
                const selectedActionsData = actionIds.map(id => actions.find(a => a.id === id.trim())).filter(Boolean) as Action[];
                setSelectedActions(selectedActionsData);
            }
        }
    }, [currentAction, form, isAddAction]);




    const actionTypeOptions = [
        { value: "file", label: "Open File", icon: "description" },
        { value: "directory", label: "Open Directory", icon: "folder" },
        { value: "command", label: "Execute Command", icon: "terminal" },
        { value: "url", label: "Open URL", icon: "link" },
        { value: "notice", label: "Show Notice", icon: "notifications" },
        { value: "group", label: "Action Group", icon: "view_list" },
    ];
    const onSubmit = async (values: z.infer<typeof formSchema>) => {
        if (isSubmitting) return;
        
        setIsSubmitting(true);
        
        try {
            // 处理args字段
            const args = values.args?.split(",").map(arg => arg.trim()).filter(arg => arg.length > 0);
            
            // 构建actionData，过滤掉undefined的可选字段
            const actionData: any = {
                name: values.name,
                desc: values.desc || "",
                wait: values.wait,
                type: values.type as Action['type'],
                command: values.type === "group" ? "group" : values.command,
                args: args && args.length > 0 ? args : undefined,
            };
            
            // 只有当值存在且大于0时才添加retry和timeout
            if (values.retry !== undefined && values.retry > 0) {
                actionData.retry = values.retry;
            }
            if (values.timeout !== undefined && values.timeout > 0) {
                actionData.timeout = values.timeout;
            }
            
            if (isAddAction) {
                await createAction(actionData);
                toast.success("Action created successfully!");
                navigate("/action");
            } else {
                await updateAction(id!, actionData);
                toast.success("Action updated successfully!");
                navigate("/action");
            }
        } catch (error) {
            console.error('Action submission error:', error);
            const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
            toast.error(`Failed to ${isAddAction ? 'create' : 'update'} action: ${errorMessage}`);
        } finally {
            setIsSubmitting(false);
        }
    }

    const handleSelectFile = useCallback(async () => {
        const actionType = form.getValues("type");
        const isFile = actionType === "file";
        const path = await open_file_select_dialog(isFile);
        if (path){
            form.setValue("command", path);
        }
    }, [form]);

    // 根据action类型获取字段标签和占位符
    const getFieldLabelsAndPlaceholders = useCallback((type: ActionTypeValue) => {
        switch (type) {
            case "notice":
                return {
                    commandLabel: "Notice Title",
                    commandPlaceholder: "Enter notice title",
                    argsLabel: "Notice Body",
                    argsPlaceholder: "Enter notice body (use comma for line breaks)"
                };
            case "group":
                return {
                    commandLabel: "Group Placeholder",
                    commandPlaceholder: "group (auto-filled)",
                    argsLabel: "Action IDs",
                    argsPlaceholder: "Enter action IDs (comma separated)"
                };
            case "command":
                return {
                    commandLabel: "Command",
                    commandPlaceholder: "Enter command",
                    argsLabel: "Arguments",
                    argsPlaceholder: "Enter arguments (comma separated)"
                };
            case "url":
                return {
                    commandLabel: "URL",
                    commandPlaceholder: "Enter URL",
                    argsLabel: "Parameters",
                    argsPlaceholder: "Enter URL parameters (comma separated)"
                };
            case "file":
            case "directory":
            default:
                return {
                    commandLabel: "Path",
                    commandPlaceholder: "Enter file/directory path",
                    argsLabel: "Arguments",
                    argsPlaceholder: "Enter arguments (comma separated)"
                };
        }
    }, []);

    const fieldLabels = useMemo(() => getFieldLabelsAndPlaceholders(currentActionType), [currentActionType, getFieldLabelsAndPlaceholders]);

    // 重置表单
    const handleReset = useCallback(() => {
        form.reset({
            name: "",
            desc: "",
            command: "",
            args: "",
            wait: 0,
            retry: 0,
            timeout: 0,
            type: "file"
        });
        setCurrentActionType("file");
        setSelectedActions([]);
    }, [form]);

    // 处理选中的actions变化
    const handleActionsChange = useCallback((actions: Action[]) => {
        setSelectedActions(actions);
        // 将选中的action IDs设置到args字段
        const actionIds = actions.map(action => action.id).join(',');
        form.setValue('args', actionIds);
    }, [form]);


    return (
        <>
        <Card className="action-modify flex flex-col h-[calc(100vh-8rem)]" >
            <CardHeader className="flex-shrink-0">
                <CardTitle>{isAddAction ? "Add Action" : "Modify Action"}</CardTitle>
                <CardAction className='cursor-pointer' onClick={() => navigate("/action")}>
                    <span className="material-symbols-outlined">
                        arrow_back
                    </span>
                </CardAction>
            </CardHeader>
            <CardContent className="flex-1 overflow-y-auto min-h-0 custom-scrollbar">
                <Form {...form} >
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                        className="h-full"
                    >
                        <div className='grid gap-4 grid-cols-2 mb-1'>
                            <FormField
                                control={form.control}
                                name="name"
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Action Name</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter action name"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />

                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="type"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Action Type</FormLabel>
                                        <Select value={field.value || "file"} onValueChange={(e) => {
                                            if (e && e != '') {
                                                field.onChange(e)
                                                handleTypeChange(e as ActionTypeValue)
                                            }
                                        }} disabled={!isAddAction}>
                                            <FormControl>
                                                <SelectTrigger className={`w-5/6 ${!isAddAction ? 'opacity-50 cursor-not-allowed' : ''}`}>
                                                    <SelectValue placeholder="Select action type" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                <SelectGroup>
                                                    <SelectLabel>Action Types</SelectLabel>
                                                    {actionTypeOptions.map((option) => (
                                                        <SelectItem key={option.value} value={option.value}>
                                                            <div className="flex items-center gap-2">
                                                                <span className="material-symbols-outlined text-sm">
                                                                    {option.icon}
                                                                </span>
                                                                {option.label}
                                                            </div>
                                                        </SelectItem>
                                                    ))}
                                                </SelectGroup>
                                            </SelectContent>
                                        </Select>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                        </div>
                        <FormField
                            control={form.control}
                            name="desc"
                            render={({ field }) => (
                                <FormItem className='w-5/6 mb-2'>
                                    <FormLabel className="block text-sm font-medium text-gray-700">Desc</FormLabel>
                                    <FormControl>
                                        <Textarea
                                            {...field}
                                            placeholder="Enter action desc"
                                            className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 resize-none"
                                        />
                                    </FormControl>
                                    <p className="text-muted-foreground text-sm">
                                        Describe the action.
                                    </p>
                                </FormItem>
                            )}
                        />
                        <div className='grid gap-4 grid-cols-1 '>
                            {currentActionType !== "group" && (
                                <FormField
                                    name="command"
                                    control={form.control}
                                    render={({ field }) => (
                                        <FormItem className='mb-2'>
                                            <FormLabel className="block text-sm font-medium text-gray-700">
                                                {fieldLabels.commandLabel}
                                            </FormLabel>
                                            <div className="flex items-center space-x-2">
                                                <FormControl>
                                                    <Input
                                                        {...field}
                                                        placeholder={fieldLabels.commandPlaceholder}
                                                        className="flex-1"
                                                    />
                                                </FormControl>
                                                {(currentActionType === "file" || currentActionType === "directory") && (
                                                    <Button
                                                        type="button"
                                                        variant="outline"
                                                        size="sm"
                                                        onClick={handleSelectFile}
                                                        className="px-3"
                                                    >
                                                        <span className="material-symbols-outlined text-sm">folder_open</span>
                                                    </Button>
                                                )}
                                            </div>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                            )}
                            {(currentActionType === "command" || currentActionType === "notice" || currentActionType === "group") && (
                                <FormField
                                    name="args"
                                    control={form.control}
                                    render={({ field }) => (
                                        <FormItem className='w-5/6 mb-2'>
                                            <FormLabel className="block text-sm font-medium text-gray-700">
                                                {fieldLabels.argsLabel}
                                            </FormLabel>
                                            <FormControl>
                                                {currentActionType === "notice" ? (
                                                    <Textarea
                                                        {...field}
                                                        placeholder={fieldLabels.argsPlaceholder}
                                                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 resize-none"
                                                        rows={3}
                                                    />
                                                ) : currentActionType === "group" ? (
                                    <div className="flex gap-5">
                                        <div className="w-48 space-y-2">
                                            <Button
                                                type="button"
                                                variant="outline"
                                                onClick={() => setIsActionSelectOpen(true)}
                                                className="w-full justify-start"
                                            >
                                                <span className="material-symbols-outlined text-sm mr-2">add</span>
                                                选择Actions ({selectedActions.length})
                                            </Button>
                                            <Input
                                                {...field}
                                                type="hidden"
                                            />
                                        </div>
                                        {selectedActions.length > 0 && (
                                            <div className=" w-4/5 border rounded-md p-3 bg-gray-50 flex flex-col max-h-96">
                                                <div className="text-sm font-medium text-gray-700 mb-2 flex items-center justify-between flex-shrink-0">
                                                    <span>已选择的Actions ({selectedActions.length})</span>
                                                    <span className="text-xs text-gray-500">拖拽重新排序</span>
                                                </div>
                                                <div className="flex-1 overflow-y-auto space-y-1 min-h-0">
                                                    {selectedActions.map((action, index) => {
                                                        const isValid = actions.some(a => a.id === action.id);
                                                        return (
                                                            <div 
                                                                key={action.id} 
                                                                className={`flex items-center justify-between py-2 px-3 rounded border transition-colors ${
                                                                    isValid 
                                                                        ? 'bg-white border-gray-200 hover:border-gray-300' 
                                                                        : 'bg-red-50 border-red-200 hover:border-red-300'
                                                                }`}
                                                            >
                                                                <div className="flex items-center gap-2 flex-1 min-w-0">
                                                                    <span className="text-xs text-gray-400 font-mono w-6 text-center">
                                                                        {index + 1}
                                                                    </span>
                                                                    <div className="flex-1 min-w-0">
                                                                        <div className={`text-sm truncate ${isValid ? 'text-gray-900' : 'text-red-600'}`}>
                                                                            {action.name}
                                                                        </div>
                                                                        {!isValid && (
                                                                            <div className="text-xs text-red-500">
                                                                                Action已被删除
                                                                            </div>
                                                                        )}
                                                                    </div>
                                                                </div>
                                                                <div className="flex items-center gap-1">
                                                                    {index > 0 && (
                                                                        <Button
                                                                            type="button"
                                                                            variant="ghost"
                                                                            size="sm"
                                                                            onClick={() => {
                                                                                const newActions = [...selectedActions];
                                                                                [newActions[index], newActions[index - 1]] = [newActions[index - 1], newActions[index]];
                                                                                handleActionsChange(newActions);
                                                                            }}
                                                                            className="h-6 w-6 p-0 hover:bg-gray-200"
                                                                            title="上移"
                                                                        >
                                                                            <span className="material-symbols-outlined text-xs">keyboard_arrow_up</span>
                                                                        </Button>
                                                                    )}
                                                                    {index < selectedActions.length - 1 && (
                                                                        <Button
                                                                            type="button"
                                                                            variant="ghost"
                                                                            size="sm"
                                                                            onClick={() => {
                                                                                const newActions = [...selectedActions];
                                                                                [newActions[index], newActions[index + 1]] = [newActions[index + 1], newActions[index]];
                                                                                handleActionsChange(newActions);
                                                                            }}
                                                                            className="h-6 w-6 p-0 hover:bg-gray-200"
                                                                            title="下移"
                                                                        >
                                                                            <span className="material-symbols-outlined text-xs">keyboard_arrow_down</span>
                                                                        </Button>
                                                                    )}
                                                                    <Button
                                                                        type="button"
                                                                        variant="ghost"
                                                                        size="sm"
                                                                        onClick={() => {
                                                                            const newActions = selectedActions.filter(a => a.id !== action.id);
                                                                            handleActionsChange(newActions);
                                                                        }}
                                                                        className={`h-6 w-6 p-0 ${isValid ? 'hover:bg-red-100 hover:text-red-600' : 'hover:bg-red-200 text-red-600'}`}
                                                                        title="删除"
                                                                    >
                                                                        <span className="material-symbols-outlined text-xs">close</span>
                                                                    </Button>
                                                                </div>
                                                            </div>
                                                        );
                                                    })}
                                                </div>
                                                {selectedActions.some(action => !actions.some(a => a.id === action.id)) && (
                                                    <div className="mt-2 p-2 bg-yellow-50 border border-yellow-200 rounded text-xs text-yellow-700 flex-shrink-0">
                                                        <span className="material-symbols-outlined text-xs mr-1">warning</span>
                                                        检测到无效的Action，建议删除或重新选择
                                                    </div>
                                                )}
                                            </div>
                                        )}
                                    </div>
                                                ) : (
                                                    <Input
                                                        {...field}
                                                        type="text"
                                                        placeholder={fieldLabels.argsPlaceholder}
                                                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                                    />
                                                )}
                                            </FormControl>
                                        </FormItem>
                                    )}
                                />
                            )}
                        </div>
                        <div className="grid gap-4 grid-cols-3">
                            <FormField
                                name="wait"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Wait (milliseconds)</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="number"
                                                min="0"
                                                placeholder="0"
                                                className="flex-1"
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="retry"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Retry Count</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="number"
                                                min="0"
                                                placeholder="0"
                                                className="flex-1"
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="timeout"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Timeout (seconds)</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="number"
                                                min="0"
                                                placeholder="0"
                                                className="flex-1"
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                        </div>
                        <div className="flex mt-5 gap-3">
                            <Button 
                                type="button" 
                                variant="outline" 
                                onClick={() => navigate("/action")}
                                disabled={isSubmitting}
                                className="flex-1"
                            >
                                Cancel
                            </Button>
                            <Button 
                                type="button" 
                                variant="outline" 
                                onClick={handleReset}
                                disabled={isSubmitting}
                                className="flex-1 flex items-center justify-center gap-2"
                            >
                                <span className="material-symbols-outlined text-sm">
                                    refresh
                                </span>
                                Reset
                            </Button>
                            <Button 
                                type="submit" 
                                className="flex-[2] cursor-pointer"
                                disabled={isSubmitting}
                            >
                                {isSubmitting ? (
                                    <>
                                        <span className="material-symbols-outlined animate-spin text-sm mr-2">
                                            progress_activity
                                        </span>
                                        {isAddAction ? "Creating..." : "Updating..."}
                                    </>
                                ) : (
                                    <>
                                        <span className="material-symbols-outlined text-sm mr-2">
                                            {isAddAction ? "add" : "edit"}
                                        </span>
                                        {isAddAction ? "Create Action" : "Update Action"}
                                    </>
                                )}
                            </Button>
                        </div>
                    </form>
                </Form>
            </CardContent>
        </Card>
        
        {/* ActionSelect Modal */}
        {isActionSelectOpen && (
            <div className="fixed inset-0 bg-black/60 backdrop-blur-sm animate-[fadeIn_0.2s_ease-out] flex items-center justify-center z-50">
                <div className="bg-white rounded-lg p-6 w-full max-w-4xl max-h-[80vh] overflow-hidden">
                    <div className="flex justify-between items-center mb-4">
                        <h3 className="text-lg font-semibold">选择Actions</h3>
                        <Button
                            type="button"
                            variant="ghost"
                            onClick={() => setIsActionSelectOpen(false)}
                            className="h-8 w-8 p-0"
                        >
                            <span className="material-symbols-outlined">close</span>
                        </Button>
                    </div>
                    <div className="h-96 overflow-y-auto custom-scrollbar">
                        <ActionSelect
                            selectedActions={selectedActions}
                            onActionsChange={handleActionsChange}
                            multiSelect={true}
                            maxHeight="100%"
                        />
                    </div>
                    <div className="flex justify-end gap-2 mt-4">
                        <Button
                            type="button"
                            variant="outline"
                            onClick={() => setIsActionSelectOpen(false)}
                        >
                            取消
                        </Button>
                        <Button
                            type="button"
                            onClick={() => setIsActionSelectOpen(false)}
                        >
                            确定 ({selectedActions.length})
                        </Button>
                    </div>
                </div>
            </div>
        )}
        </>
    );
};

export default ActionModify;

