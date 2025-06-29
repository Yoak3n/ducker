import type { Task } from "@/types";

import TaskItem from "../TaskItem";

interface Props {
    tasks: Task[],
    changeTask: (id: string) => void
}

export default function TaskList({ tasks,changeTask }: Props) {


    const handleTaskChange = (id: string) => {
        changeTask(id)
    }
    const sortedTasks = [...tasks].sort((a, b) => {
        if (a.completed && !b.completed) return 1;
        if (!a.completed && b.completed) return -1;
        return 0;
    });

    return <ul className="task-list">
        {sortedTasks.map(task => (
            <TaskItem key={task.id} task={task} changeTask={handleTaskChange} />
        ))}
    </ul>
}