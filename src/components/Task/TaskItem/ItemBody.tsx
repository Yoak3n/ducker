import { Checkbox } from "@/components/ui/checkbox";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

import type { Task } from "@/types";
import { useState } from "react";
import TaskItem from "./index";
import { formatHourAndMinute } from "@/utils/date";
import { t } from "i18next";
import { Play, ChevronDown } from "lucide-react";

interface ItemBodyProps {
    root?: boolean;
    task: Task;
    changeTask: (id: string, sub?: boolean) => void;
}

const ItemBody = ({ root, task, changeTask }: ItemBodyProps) => {
    const [isExpanded, setIsExpanded] = useState(false);
    // Keeps the subtree mounted through the collapse transition so it
    // eases out instead of teleporting away.
    const [collapsePending, setCollapsePending] = useState(false);
    const toggleExpanded = () => {
        if (isExpanded) {
            setCollapsePending(true);
            // Match the 220ms grid-rows collapse in TaskItem/index.css
            window.setTimeout(() => {
                setCollapsePending(false);
                setIsExpanded(false);
            }, 220);
        } else {
            setIsExpanded(true);
        }
    };
    return (
        <div className="flex items-center w-full ">
            <Checkbox
                checked={task.completed}
                onCheckedChange={() => root ? changeTask(task.id) : changeTask(task.id, true)}
            />
            <div className="flex justify-between w-full">
                {/* Hover hint only — execution lives in the context menu,
                    not on left-click. */}
                <Tooltip>
                    <TooltipTrigger>
                        <div className="cursor-default p-2.5 text-base select-text">
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
                        <div className="flex items-center gap-2 text-sm text-brand cursor-default">
                            <Play size={14} />
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
                <button className="dropdown-button" onClick={toggleExpanded}>
                    <span className="dropdown-chevron" data-expanded={isExpanded}>
                        <ChevronDown size={16} />
                    </span>
                </button>}
            {(isExpanded || collapsePending) && task.children &&
                <ul className="sub-task-list" data-collapsed={!isExpanded}>
                    {task.children.map((subTask) => (
                        <TaskItem key={subTask.id} root={false} task={subTask} changeTask={changeTask} />
                    ))}
                </ul>}
        </div>
    )
};

export default ItemBody;
