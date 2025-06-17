import { useState } from "react";
import type { Task } from "@/types";
import "./index.css";
interface Props {
    tasks: Task[]
}
// const toggleTaskCompletion = (period: TabType, day: DayOfWeek | null, taskId: number) => {
//     setTasks(prevTasks => {

//         const newTasks: TasksData = { ...prevTasks };

//         if (period === 'today' || period === 'monthly') {
//             newTasks[period] = newTasks[period].map(task =>
//                 task.id === taskId ? { ...task, completed: !task.completed } : task
//             );
//         } else if (period === 'weekly' && day) {
//             newTasks.weekly = { ...newTasks.weekly };
//             newTasks.weekly[day] = newTasks.weekly[day].map(task =>
//                 task.id === taskId ? { ...task, completed: !task.completed } : task
//             );
//         }

//         return newTasks;
//     });
// };


const TodayView = (props: Props) => {
    const { tasks } = props;
    const [taskList, setTaskList] = useState<Task[]>(tasks);
    const completedCount = taskList.filter(t => t.completed).length;
    const totalCount = taskList.length;
    const progressPercent = totalCount > 0 ? (completedCount / totalCount) * 100 : 0;

    const toggleTaskCompletion = (taskId: number) => {
        setTaskList(prevTaskList =>
            prevTaskList.map(task =>
                task.id === taskId ? { ...task, completed: !task.completed } : task
            )
        );
    }


    return (
        <div className="today-view">
            <h2>今日任务 ({new Date().toLocaleDateString()})</h2>
            <div className="progress-bar">
                <div
                    className="progress"
                    style={{ width: `${progressPercent}%` }}
                ></div>
                <span>
                    {completedCount} / {totalCount} 完成
                </span>
            </div>
            <ul className="task-list">
                {taskList.map(task => (
                    <li key={task.id} className={task.completed ? 'completed task-item' : 'task-item'}>
                        <input
                            type="checkbox"
                            checked={task.completed}
                            onChange={() => toggleTaskCompletion(task.id)}
                        />
                        <div className="task-item-title"  onClick={() => toggleTaskCompletion(task.id)}>
                            {task.title}
                        </div>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default TodayView;