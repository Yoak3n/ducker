import type { Task } from "@/types";

import TaskItem from "../TaskItem";
import { extractTimeStampSecond } from "@/utils";

interface Props {
    tasks: Task[],
    todayDate: Date,
    todayRange: {
        start: number,
        end: number
    },
    changeTask: (id: string) => void
    variant?: "today" | "weekly";
}

export default function TaskList({ tasks,todayDate,todayRange,changeTask,variant }: Props) {
    const handleTaskChange = (id: string) => {
        changeTask(id)
    }
    const sortedTasks = [...tasks].sort((a, b) => {
        if (extractTimeStampSecond(a.due_to!) < extractTimeStampSecond(b.due_to!)) return 1;
        if (!a.completed && b.completed) return -1;
        if (a.completed && !b.completed) return 1;
        if (extractTimeStampSecond(a.due_to!) > extractTimeStampSecond(b.due_to!)) return -1;
        return 0;
    });

    return <ul className="task-list max-h-96 overflow-y-auto">
        {sortedTasks.map(task => (
            <TaskItem
            variant={variant}
             key={task.id} 
             task={task} 
             changeTask={handleTaskChange} 
             todayDate={todayDate} 
             today={extractTimeStampSecond(task.due_to!) >= todayRange.start && extractTimeStampSecond(task.due_to!) <= todayRange.end} />
        ))}
    </ul>
}