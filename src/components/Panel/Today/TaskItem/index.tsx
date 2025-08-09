import  TaskItem from "@/components/Task/TaskItem"
import type{ Task } from "@/types";
import { extractTimeStampSecond } from "@/utils";

interface Props {
    root?: boolean;
    task: Task,
    today: boolean,
    changeTask: (id: string,sub?:boolean) => void;
    todayDate: Date

}

export default function TodayTaskItem({root,task,changeTask,today,todayDate}:Props) {
    return <TaskItem 
    root={root} 
    task={task} 
    changeTask={changeTask} 
    addedClassName={`${today ? "today" : ""} ${extractTimeStampSecond(task.due_to!) <= todayDate.getTime()/1000 ? "dued" : ""} ${task.completed ? "completed" : ""}`} />
}
