import { useEffect, useState, type FC } from "react";
import { useParams } from "react-router-dom";
import { emit } from "@tauri-apps/api/event";

import { TaskForm } from "@/components/Task";
import type { Task, TaskData, PeriodicTaskData } from '@/types';
import { PeriodicFromNumber } from '@/types/modules/task';

import { useTaskStore } from "@/store";
import { create_periodic_task, update_periodic_task } from "@/api";

const TaskModify: FC = () => {
    const { id } = useParams<{ id?: string }>();
    const { fetchTasks, tasks, updateTask, createTask } = useTaskStore(state => state)
    const [editingTask, setEditingTask] = useState<Task | undefined>(undefined)

    // 如果有id参数，说明是编辑模式；否则是新建模式
    const [isEditMode, setIsEditMode] = useState(false);
    useEffect(() => {
        fetchTasks()
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    useEffect(() => {
        setIsEditMode(!!id);
        if (id){
            setIsEditMode(true);
        }else{
            setIsEditMode(false);
        }
        if (tasks.length > 0) {
            const task = tasks.find(task => task.id === id)
            setEditingTask(task)
        } else if (!isEditMode) {
            setEditingTask(undefined)
        }
    }, [id, tasks])

    const modifyTask = async (taskData: TaskData, periodic?: number) => {
        try {
            if (periodic) {
                const periodicTask: PeriodicTaskData = {
                    name: taskData.name,
                    interval: PeriodicFromNumber(periodic),
                    task: taskData,
                };
                if (isEditMode) {
                    // 编辑模式下，调用更新任务接口
                    await update_periodic_task(id!, periodicTask);
                } else {
                    // 新建模式下，调用创建任务接口
                    await create_periodic_task(periodicTask);
                    console.log('任务创建成功:', taskData);
                }
            } else {
                if (isEditMode) {
                    // 编辑模式下，调用更新任务接口
                    await updateTask(id!, taskData);
                } else {
                    // 新建模式下，调用创建任务接口
                    await createTask(taskData);
                    console.log('任务创建成功:', taskData);
                }
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