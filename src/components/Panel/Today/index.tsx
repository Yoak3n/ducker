import { useEffect, useState } from "react";
import type { Task, TaskData } from "@/types";
import TaskList from "@/components/Task/TaskList";
import { TaskModal } from "@/components/Task";
import { Button } from "@/components/ui/button";
import { useTaskStore } from "@/store";
import { extractDateViaDateObject } from "@/utils";

const TodayView = () => {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [parentTask, setParentTask] = useState<Task | null>(null);
    const [editingTask, setEditingTask] = useState<Task | null>(null);

    const taskStore = useTaskStore()
    const todayDate = new Date().toLocaleDateString()
    useEffect(() => {
        taskStore.fetchTasks()
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])
    // const [taskList, setTaskList] = useState<Task[]>(tasks);
    const tasksList = taskStore.tasks.filter(task => extractDateViaDateObject(task.due_to!) === todayDate)
    let completedValueCount = 0
    taskStore.tasks.forEach(item => {
        if (item.completed) completedValueCount += item.value || 0;
        if (item.children) {
            item.children.filter(child => {
                if (child.completed) completedValueCount += child.value || 0;
            })
        }
    });
    let totalCount = 0
    tasksList.forEach((item) => {
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

    const handleSaveTask = (taskData: Partial<TaskData>) => {
        if (editingTask) {
            // 编辑模式
            // TODO: 开始写前后端交互吧！
            // setTaskList(prev => prev.map(task =>
            //     task.id === editingTask.id
            //         ? { ...task, ...taskData }
            //         : task
            // ));
        } else {
            // 创建模式
            const newTask: TaskData = {
                completed: false,
                name: taskData.name!,
                ...taskData
            };
            console.log(newTask)
            taskStore.createTask(newTask)

        }
    };

    const handleTaskStatueChange = (id: string) => {
        taskStore.toggleTaskCompletion(id)

    }

    return (
        <div className="today-view">
            <TaskModal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                onSave={handleSaveTask}
                task={editingTask}
                parentTask={parentTask}
            />
            <div className="today-title" style={{display: 'flex', justifyContent: 'space-between'}}>
                <h2>今日任务 ({todayDate})</h2>
                <Button variant="outline" onClick={handleCreateTask}>创建任务</Button>
            </div>
            {totalCount > 0 &&
                <div className="progress-bar">
                    <div className="progress"
                        style={{ width: `${progressPercent}%` }}
                    ></div>
                    <span>
                        {completedValueCount} / {totalCount} 完成
                    </span>
                </div>
            }

            <TaskList tasks={tasksList} changeTask={handleTaskStatueChange} />
        </div>
    );
};

export default TodayView;