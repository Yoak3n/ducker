import type { Task } from "@/types";
import { ContextMenuItem,ContextMenuLabel } from "@/components/ui/context-menu";
import { showWindow } from "@/api";

const modifyTask = async (taskId: string) => {
    await showWindow("task","/task/" + taskId);
}


const ContextItem = ({ task }: { task: Task}) => {
    return (
        <div>
            <ContextMenuLabel>{task.name}</ContextMenuLabel>
            <ContextMenuItem onClick={() => modifyTask(task.id)}>
                编辑
            </ContextMenuItem>
            <ContextMenuItem>
                创建子任务
            </ContextMenuItem>
            <ContextMenuItem variant="destructive">
                删除
            </ContextMenuItem>
        </div>
    )
}

export default ContextItem;