import { Checkbox } from "@/components/ui/checkbox";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

import { execute_actions } from "@/api";
import type { Task } from "@/types";
import { useState } from "react";
import TaskItem from "./index";
import { formatHourAndMinute } from "@/utils/date";

interface ItemBodyProps {
    root?: boolean;
    task: Task;
    changeTask: (id: string, sub?: boolean) => void;
}

const ItemBody = ({ root, task, changeTask }: ItemBodyProps) => {
    const [isExpanded, setIsExpanded] = useState(false);
    return (
        <div className="task-item">
            <Checkbox
                checked={task.completed}
                onCheckedChange={() => root ? changeTask(task.id) : changeTask(task.id, true)}
            />
            <div className="task-item-content">
                <Tooltip>
                    <TooltipTrigger onClick={(e) => { e.preventDefault(); execute_actions(task.actions) }}>
                        <div className="task-item-title">
                            {task.name}
                        </div>
                    </TooltipTrigger>
                    {
                        task.actions && task.actions.length > 0 &&
                        <TooltipContent>
                            {task.actions?.map((action) => (
                                <div key={action.id}>
                                    {action.name}
                                </div>
                            ))}
                        </TooltipContent>
                    }
                </Tooltip>
                {task.auto && <div className="task-item-tail">
                    <span className="material-symbols-outlined">
                        autoplay
                    </span>
                    {formatHourAndMinute(task.due_to || "")}
                </div>}
            </div>
            {task.children && task.children.length > 0 &&
                <button className="dropdown-button" onClick={() => { setIsExpanded(!isExpanded) }}>
                    <span className="material-symbols-outlined">
                        {isExpanded ? "arrow_drop_up" : "arrow_drop_down"}
                    </span>
                </button>}
            {isExpanded && task.children &&
                <ul className="sub-task-list">
                    {task.children.map((subTask) => (
                        <TaskItem key={subTask.id} root={false} task={subTask} changeTask={changeTask} />
                    ))}
                </ul>}
        </div>
    )
};

export default ItemBody;