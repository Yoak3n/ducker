import { useState } from "react";

import type { Task } from "@/types";
import TaskList from "@/components/Task/TaskList";

interface Props {
    tasks: Task[]
}


const TodayView = (props: Props) => {
    const { tasks } = props;
    const [taskList, setTaskList] = useState<Task[]>(tasks);
    let completedCount = 0
    taskList.forEach(item=>{
        if (item.completed) completedCount += 1
        if (item.children){
            item.children.filter(child=>{
                if (child.completed) completedCount += 1
            })
        }
    });
    let totalCount = 0 
    taskList.forEach((item)=>{
        totalCount += 1;
        if(item.children){
            totalCount += item.children.length;
        }
    });
    const progressPercent = totalCount > 0 ? (completedCount / totalCount) * 100 : 0;

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
            <TaskList tasks={taskList} setTasks={setTaskList} />
        </div>
    );
};

export default TodayView;