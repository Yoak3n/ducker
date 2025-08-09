import { useState } from "react";

import { Checkbox } from "@/components/ui/checkbox";

import type { Task } from "@/types";
import { execute_actions } from "@/api";
import "./index.css";

interface Props {
    root?: boolean;
    task: Task,
    changeTask: (id: string,sub?:boolean) => void;
    addedClassName?: string;
}

export default function TaskItem({ root = true, task,changeTask,addedClassName }: Props) {
    const [isExpanded, setIsExpanded] = useState(false);
    const itemClassName = (root ? "root-task" : "sub-task") +" "+ (task.completed ? "completed" : "")
    return (
        <li className={itemClassName + " " + addedClassName} key={task.id}>
            <div className="task-item">
                <Checkbox
                    checked={task.completed}
                    onCheckedChange={() => {
                        return root? changeTask(task.id):changeTask(task.id,true)
                    }

                    }
                />
                <div className="task-item-content" onClick={(e)=>{e.preventDefault();execute_actions(task.actions)}}>
                    <div className="task-item-title">{task.name}</div>
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
                        <TaskItem key={subTask.id} root={false} task={subTask} changeTask={changeTask}/>
                    ))}
                </ul>}
        </li>
    )
}