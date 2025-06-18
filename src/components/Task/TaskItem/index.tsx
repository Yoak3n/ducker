import { useState } from "react";
import type { Task } from "@/types";

import "./index.css";

interface Props {
    root?: boolean;
    task: Task,
    changeTask: (id: number,sub?:boolean) => void;

}

export default function TaskItem({ root = true, task,changeTask }: Props) {
    const [isExpanded, setIsExpanded] = useState(false);
    const itemClassName = (root ? "root-task" : "sub-task") +" "+ (task.completed ? "completed" : "")
    return (
        <li className={itemClassName } key={task.id}>
            <div className="task-item">
                <input
                    type="checkbox"
                    checked={task.completed}
                    onChange={() => root? changeTask(task.id):changeTask(task.id,true)}
                />
                <div className="task-item-content" >
                    <div className="task-item-title">{task.title}</div>
                </div>
                {task.children && task.children.length > 0 &&
                    <button className="dropdown-button" onClick={() => { setIsExpanded(!isExpanded) }}>
                        <span className="material-symbols-outlined">
                            {isExpanded ? "arrow_drop_up" : "arrow_drop_down"}
                        </span>
                    </button>}
            </div>
            {isExpanded && task.children &&
                <ul className="sub-task-list">
                    {task.children.map((subTask) => (
                        <TaskItem key={subTask.id} root={false} task={subTask} changeTask={changeTask} />
                    ))}
                </ul>}
        </li>
    )
}