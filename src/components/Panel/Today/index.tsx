import { useState } from "react";
import "./index.css";
import type { Task } from "@/types";
import TaskList from "@/components/Task/TaskList";
import { TaskModal } from "@/components/Task";
import { Button } from "@/components/ui/button";

interface Props {
    tasks: Task[]
}


const TodayView = ({ tasks }: Props) => {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [parentTask, setParentTask] = useState<Task | null>(null);
    const [editingTask, setEditingTask] = useState<Task | null>(null);
    // TODO 之后用 useContext 来获取任务数据
    const [taskList, setTaskList] = useState<Task[]>(tasks);

    let completedValueCount = 0
    taskList.forEach(item => {
        if (item.completed) completedValueCount += item.value || 0;
        if (item.children) {
            item.children.filter(child => {
                if (child.completed) completedValueCount += child.value || 0;
            })
        }
    });
    let totalCount = 0
    taskList.forEach((item) => {
        totalCount += item.value || 0;
        if (item.children) {
            item.children.forEach(child => {
                totalCount += child.value || 0;
            });
        }
    });
    const progressPercent = totalCount > 0 ? (completedValueCount / totalCount) * 100 : 0;

    const handleCreateTask = () => {
        setEditingTask(null);
        setParentTask(null);
        setIsModalOpen(true);
    };
    // const handleEditTask = (task: Task) => {
    //     setEditingTask(task);
    //     setParentTask(null);
    //     setIsModalOpen(true);
    // };
    // const handleCreateSubTask = (parent: Task) => {
    //     setEditingTask(null);
    //     setParentTask(parent);
    //     setIsModalOpen(true);
    // };

    const handleSaveTask = (taskData: Partial<Task>) => {
        if (editingTask) {
            // 编辑模式
            setTaskList(prev => prev.map(task =>
                task.id === editingTask.id
                    ? { ...task, ...taskData }
                    : task
            ));
        } else {
            // 创建模式
            const newTask: Task = {
                id: Date.now(),
                create_at: new Date(),
                completed: false,
                auto: false,
                ...taskData
            } as Task;

            if (parentTask) {
                // 创建子任务
                setTaskList(prev => prev.map(task => {
                    if (task.id === parentTask.id) {
                        return {
                            ...task,
                            children: [...(task.children || []), newTask]
                        };
                    }
                    return task;
                }));
            } else {
                // 创建根任务
                setTaskList(prev => [...prev, newTask]);
            }
        }
    };


    return (
        <div className="today-view">
            <TaskModal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                onSave={handleSaveTask}
                task={editingTask}
                parentTask={parentTask}
            />
            <div className="today-title">
                <h2>今日任务 ({new Date().toLocaleDateString()})</h2>
                <Button variant="outline" onClick={handleCreateTask}>创建任务</Button>
            </div>

            <div className="progress-bar">
                <div
                    className="progress"
                    style={{ width: `${progressPercent}%` }}
                ></div>
                <span>
                    {completedValueCount} / {totalCount} 完成
                </span>
            </div>
            <TaskList tasks={taskList} setTasks={setTaskList} />
        </div>
    );
};

export default TodayView;