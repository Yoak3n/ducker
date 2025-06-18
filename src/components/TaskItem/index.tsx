import { useState } from "react";
import type { Task } from "@/types";

import "./index.css";

interface Props {
    root?: boolean;
    task: Task
}

export default function TaskItem({ root = true, task }: Props) {
    const [isCompleted, setIsCompleted] = useState(task.completed);
    const [isExpanded, setIsExpanded] = useState(false);
    const toggleTaskCompletion = () => {
        setIsCompleted(!isCompleted);
        // TODO: Update task completion status in the backend
    }
    return (
        <li className={root ? "root-task" : "sub-task"} key={task.id}>
            <div className={isCompleted ? "task-item  completed" : "task-item"}>
                <input
                    type="checkbox"
                    checked={isCompleted}
                    onChange={() => toggleTaskCompletion()}
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
            {isExpanded && task.children && task.children.length > 0 &&
                <ul className="sub-task-list">
                    {task.children.map((subTask) => (
                        <TaskItem key={subTask.id} root={false} task={subTask} />
                    ))}
                </ul>}
        </li>
    )
}