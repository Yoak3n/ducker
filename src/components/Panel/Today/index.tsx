import { useState } from "react";

import type { Task } from "@/types";
import TaskItem from "@/components/TaskItem";

interface Props {
    tasks: Task[]
}


const TodayView = (props: Props) => {
    const { tasks } = props;
    const [taskList, setTaskList] = useState<Task[]>(tasks);
    const completedCount = taskList.filter(t => t.completed).length;
    const totalCount = taskList.length;
    const progressPercent = totalCount > 0 ? (completedCount / totalCount) * 100 : 0;

    const toggleTaskCompletion = (taskId: number) => {
        setTaskList(prevTaskList =>
            prevTaskList.map(task =>
                task.id === taskId ? { ...task, completed: !task.completed } : task
            )
        );
    }


    return (
        <div className="today-view">
            <h2>今日任务 ({new Date().toLocaleDateString()})</h2>
            <div className="progress-bar">
                <div
                    className="progress"
                    style={{ width: `${progressPercent}%` }}
                ></div>
                <span>
                    {completedCount} / {totalCount} 完成
                </span>
            </div>
            <ul className="task-list">
                {taskList.map(task => (
                    <TaskItem key={task.id} task={task} />
                ))}
            </ul>
        </div>
    );
};

export default TodayView;