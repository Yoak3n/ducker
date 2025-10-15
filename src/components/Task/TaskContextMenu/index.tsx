import type{ FC, ReactNode } from "react"
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuTrigger,
} from "@/components/ui/context-menu"

import ContextMenuItem from "../ContextItems"

interface Props {
    taskId: string;
    children: ReactNode;
}

const TaskContextMenu: FC<Props> = ({taskId,children}: Props) => {
    return (
        <ContextMenu>
            <ContextMenuTrigger>
                {children}
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem taskId={taskId} />
            </ContextMenuContent>
        </ContextMenu>
    )
}

export default TaskContextMenu;