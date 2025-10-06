import { useEffect, useState, type FC } from "react";
import { useParams } from "react-router-dom";
import { emit } from "@tauri-apps/api/event";

import { TaskForm } from "@/components/Task";
import type { Task, TaskData } from '@/types';
import { useTaskStore } from "@/store";

const TaskModify: FC = () => {
    const { id } = useParams<{ id?: string }>();
    const { fetchTasks, tasks, updateTask, createTask } = useTaskStore(state => state)
    const [editingTask, setEditingTask] = useState<Task | undefined>(undefined)
    
    // 如果有id参数，说明是编辑模式；否则是新建模式
    const isEditMode = !!id;
    
    useEffect(() => {
        fetchTasks()
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    useEffect(() => {
        if (isEditMode && tasks.length > 0) {
            const task = tasks.find(task => task.id === id)
            setEditingTask(task)
        } else if (!isEditMode) {
            setEditingTask(undefined)
        }
    }, [isEditMode, id, tasks])

    const modifyTask = async (taskData: TaskData) => {
        try {
            if (isEditMode) {
                // 编辑模式下，调用更新任务接口
                await updateTask(id, taskData);
                console.log('任务更新成功:', taskData);
            } else {
                // 新建模式下，调用创建任务接口
                await createTask(taskData);
                console.log('任务创建成功:', taskData);
            }
            // 发送统一的任务变更事件
            await emit('task-changed', { 
                action: isEditMode ? 'update' : 'create',
                taskId: isEditMode ? id : undefined,
                timestamp: Date.now() 
            });
        } catch (error) {
            console.error('任务操作失败:', error);
        }
    }

    return (
        <div style={{
            width: '100vw',
            height: '100vh',
            overflowY: 'auto',
            scrollbarWidth: 'none',
        }}>
            <TaskForm task={editingTask} onSave={modifyTask} />
        </div>
    )
}

export default TaskModify;