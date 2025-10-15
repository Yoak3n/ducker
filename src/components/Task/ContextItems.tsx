import { ContextMenuItem } from "@/components/ui/context-menu";
import {useTaskStore} from "@/store"
import { showWindow,execute_task } from "@/api";

const modifyTask = async (taskId: string) => {
    await showWindow("task","/task/" + taskId);
}

const ContextItems = ({ taskId }: { taskId: string}) => {
    const deleteTask = useTaskStore(state=> state.deleteTask)
    return (
        <div>
            <ContextMenuItem onClick={() => modifyTask(taskId)}>
                编辑
            </ContextMenuItem>
            <ContextMenuItem onClick={() => execute_task(taskId)}>
                运行
            </ContextMenuItem>
            <ContextMenuItem>
                创建子任务
            </ContextMenuItem>
            <ContextMenuItem variant="destructive" onClick={() => deleteTask(taskId)}>
                删除
            </ContextMenuItem>
        </div>
    )
}

export default ContextItems;