import { useEffect, useState, type FC } from "react";
import { useParams } from "react-router-dom";

import { TaskForm } from "@/components/Task";
import type { Task, TaskData, Action } from '@/types';
import { useTaskStore } from "@/store";

const TaskModify: FC = () => {
    const { id } = useParams<{ id?: string }>();
    const { fetchTasks, tasks } = useTaskStore(state => state)
    const [editingTask, setEditingTask] = useState<Task | undefined>(undefined)
    useEffect(() => {
        fetchTasks()
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    // 如果有id参数，说明是编辑模式；否则是新建模式
    const isEditMode = !!id;
    if (isEditMode) {
        const task = tasks.find(task => task.id === id)
        setEditingTask(task)
    }

    return (
        <div style={{
            width: '800px',
            height: '500px',
            overflowY: 'auto',
        }}>
            <TaskForm task={editingTask} isOpen={true} onClose={function (): void {
                throw new Error("Function not implemented.");
            }} onSave={function (taskData: TaskData): void {
                throw new Error("Function not implemented.");
            }} />
        </div>
    )
}

export default TaskModify;