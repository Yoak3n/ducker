import TaskContextMenu from "../TaskContextMenu";
import ItemBody from "./ItemBody";
import type { Task } from "@/types";

import "./index.css";


interface Props {
    root?: boolean;
    task: Task,
    changeTask: (id: string, sub?: boolean) => void;
    addedClassName?: string;
    variant?: "today" | "weekly";
}

export default function TaskItem({ root = true, task, changeTask, addedClassName, variant }: Props) {
    const itemClassName = (root ? "root-task" : "sub-task") + " " + (task.completed ? "completed" : "")
    return (
        <li className={itemClassName + " " + addedClassName} key={task.id}>
            {variant == "today" ?
                <TaskContextMenu taskId={task.id}>
                    <ItemBody task={task} changeTask={changeTask} root/>
                </TaskContextMenu> :
                <ItemBody task={task} changeTask={changeTask} root/>
            }
        </li>
    )
}