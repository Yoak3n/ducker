import type { Task } from "@/types";
import TaskItem from "../TaskItem";

interface Props {
    tasks: Task[],
    setTasks: (tasks: Task[])=>void,
}

export default function TaskList({ tasks,setTasks }: Props) {
    const handleTaskChange = (id:string,sub?:boolean)=>{
        if (sub){
            tasks.map(task=>{
                if (task.children && task.children.length > 0 ){
                    task.children = task.children.map(child=>child.id === id ? {...child,completed:!child.completed}:child)
                }
            })
            setTasks([...tasks])
        }else{
            setTasks(tasks.map(task=>{
                if (task.id === id){
                    if (task.children && task.children.length > 0 ){
                        task.children = task.children.map(child=>{
                            return {...child,completed:!child.completed}
                        })
                    }
                    return {...task,completed:!task.completed}
                }
                return task;
            }))
        }
    }
    const sortedTasks = [...tasks].sort((a, b) => {
        if (a.completed && !b.completed) return 1;
        if (!a.completed && b.completed) return -1;
        return 0;
    });
    
    return <ul className="task-list">
        {sortedTasks.map(task => (
            <TaskItem key={task.id} task={task} changeTask={handleTaskChange}/>
        ))}
    </ul>
}