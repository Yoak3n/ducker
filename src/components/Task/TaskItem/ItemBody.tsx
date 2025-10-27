import { Checkbox } from "@/components/ui/checkbox";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

import { execute_actions } from "@/api";
import type { Task } from "@/types";
import { useState } from "react";
import TaskItem from "./index";
import { formatHourAndMinute } from "@/utils/date";
import { t } from "i18next";

interface ItemBodyProps {
    root?: boolean;
    task: Task;
    changeTask: (id: string, sub?: boolean) => void;
}

const ItemBody = ({ root, task, changeTask }: ItemBodyProps) => {
    const [isExpanded, setIsExpanded] = useState(false);
    return (
        <div className="flex items-center w-full ">
            <Checkbox
                checked={task.completed}
                onCheckedChange={() => root ? changeTask(task.id) : changeTask(task.id, true)}
            />
            <div className="flex justify-between w-full">
                <Tooltip>
                    <TooltipTrigger onClick={(e) => { e.preventDefault(); execute_actions(task.actions) }}>
                        <div className="cursor-pointer p-2.5 text-base ">
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
                {task.auto && <Tooltip>
                    <TooltipTrigger>
                        <div className="flex items-center gap-2 text-sm text-[#3498db] cursor-default">
                            <span className="material-symbols-outlined">
                                autoplay
                            </span>
                            {formatHourAndMinute(task.due_to || "")}
                        </div>
                    </TooltipTrigger>
                    <TooltipContent>
                        <div>
                            {t("Auto Description")}
                        </div>
                    </TooltipContent>
                </Tooltip>}
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