import { useEffect, useState } from "react";


import { TaskForm } from "@/components/Task";
import { Button } from "@/components/ui/button";
import { Dialog, DialogTrigger } from "@/components/ui/dialog";

import { useTaskStore } from "@/store";
import { extractTimeStampSecond, getTodayRange } from "@/utils";
import {showWindow} from "@/api";
import type { Task, TaskData } from "@/types";

import TaskList from "./TaskList";
import "./index.css"



const TodayView = () => {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [parentTask, setParentTask] = useState<Task | undefined>(undefined);
    const [editingTask, setEditingTask] = useState<Task | null>(null);

    const taskStore = useTaskStore()
    const todayDate = new Date()

    const todayRange = getTodayRange()

    useEffect(() => {
        taskStore.fetchTasks()
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])
    // const [taskList, setTaskList] = useState<Task[]>(tasks);
    const tasksList = taskStore.tasks.filter(task => (extractTimeStampSecond(task.due_to!) <= todayDate.getTime() / 1000 && !task.completed) || (extractTimeStampSecond(task.due_to!) >= todayRange.start && extractTimeStampSecond(task.due_to!) <= todayRange.end))

    let completedValueCount = 0
    tasksList.forEach(item => {
        if (item.completed) completedValueCount += item.value || 0;
        if (item.children) {
            item.children.filter(child => {
                if (child.completed) completedValueCount += child.value || 0;
            })
        }
    });
    let totalCount = 0
    tasksList.forEach((item) => {
        totalCount += item.value || 0;
        if (item.children) {
            item.children.forEach(child => {
                totalCount += child.value || 0;
            });
        }
    });
    const progressPercent = totalCount > 0 ? (completedValueCount / totalCount) * 100 : 0;

    const handleCreateTask = () => {
        showWindow("task")
        setEditingTask(null);
        setParentTask(undefined);
        setIsModalOpen(true);
    };

    const handleTaskStatueChange = (id: string) => {
        taskStore.toggleTaskCompletion(id)
    }

    return (
        <div className="today-view">
            <div className="today-title flex justify-between items-center">
                <h2>今日任务 ({todayDate.toLocaleDateString()})</h2>
                <Button variant="outline" className="cursor-pointer" onClick={() => handleCreateTask()}>创建任务</Button>
            </div>
            {totalCount > 0 &&
                <div className="progress-bar">
                    <div className="progress"
                        style={{ width: `${progressPercent}%` }}
                    ></div>
                    <span>
                        {completedValueCount} / {totalCount} 完成
                    </span>
                </div>
            }
            <TaskList tasks={tasksList} todayDate={todayDate} todayRange={todayRange} changeTask={handleTaskStatueChange} />
        </div>
    );
};

export default TodayView;