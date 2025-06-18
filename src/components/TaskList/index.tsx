import type { Task } from "@/types";
import TaskItem from "../TaskItem";

interface Props {
    tasks: Task[]
}

export default function TaskList({ tasks }: Props) {
    return <ul className="task-list">
        {tasks.map(task => (
            <TaskItem key={task.id} task={task} />
        ))}
    </ul>
}