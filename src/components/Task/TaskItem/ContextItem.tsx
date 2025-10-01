import type { Task } from "@/types";
import { ContextMenuItem,ContextMenuLabel } from "@/components/ui/context-menu";
import {useTaskStore} from "@/store"
import { showWindow,execute_actions } from "@/api";

const modifyTask = async (taskId: string) => {
    await showWindow("task","/task/" + taskId);
}



const ContextItem = ({ task }: { task: Task}) => {
    const deleteTask = useTaskStore(state=> state.deleteTask)
    return (
        <div>
            <ContextMenuLabel>{task.name}</ContextMenuLabel>
            <ContextMenuItem onClick={() => modifyTask(task.id)}>
                编辑
            </ContextMenuItem>
            <ContextMenuItem onClick={() => execute_actions(task.actions)}>
                运行
            </ContextMenuItem>
            <ContextMenuItem>
                创建子任务
            </ContextMenuItem>
            <ContextMenuItem variant="destructive" onClick={() => deleteTask(task.id)}>
                删除
            </ContextMenuItem>
        </div>
    )
}

export default ContextItem;