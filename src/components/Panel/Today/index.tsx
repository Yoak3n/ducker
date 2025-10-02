import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import { Button } from "@/components/ui/button";

import { useTaskStore, type Task } from "@/store";
import { extractTimeStampSecond, getTodayRange } from "@/utils";
import { showWindow } from "@/api";
import { useI18n } from "@/hooks/use-i18n";
import TaskList from "./TaskList";
import "./index.css"



const TodayView = () => {
    const { t } = useI18n();
    const taskStore = useTaskStore()
    const todayDate = new Date()

    const todayRange = getTodayRange()

    useEffect(() => {
        taskStore.fetchTasks()
        // taskStore.fetchTodayTasks()
        // 监听任务变更事件
        let unlistenTaskChanged: Promise<() => void> | null = null;

        // 检查是否在Tauri环境中
        if (typeof window !== 'undefined') {
            try {
                unlistenTaskChanged = listen('task-changed', () => {
                    taskStore.fetchTasks();
                });
            } catch (error) {
                console.warn('事件监听器初始化失败:', error);
            }
        }

        // 清理事件监听器
        return () => {
            if (unlistenTaskChanged) {
                unlistenTaskChanged.then(fn => fn()).catch(console.warn);
            }
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])
    const [tasksList, setTasksList] = useState<Task[]>([]);
    useEffect(() => {
        setTasksList(taskStore
            .tasks
            .filter(task => (extractTimeStampSecond(task.due_to!) <= todayDate.getTime() / 1000 && !task.completed) || (extractTimeStampSecond(task.due_to!) >= todayRange.start && extractTimeStampSecond(task.due_to!) <= todayRange.end)))
    }, [taskStore.tasks])


    let completedValueCount = 0
    tasksList.forEach(item => {
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
        showWindow("task")
    };

    const handleTaskStatueChange = (id: string) => {
        console.log(id)
        taskStore.toggleTaskCompletion(id)
    }

    return (
        <div className="today-view">
            <div className="today-title flex justify-between items-center">
                <h2>{t("Today")} {todayDate.toLocaleDateString()}</h2>
                <Button variant="outline" className="cursor-pointer" onClick={() => handleCreateTask()}>
                    {t("Create Task")}
                </Button>
            </div>
            {totalCount > 0 &&
                <div className="progress-bar">
                    <div className="progress"
                        style={{ width: `${progressPercent}%` }}
                    ></div>
                    <span>
                        {completedValueCount} / {totalCount} {t("Completed")}
                    </span>
                </div>
            }
            <TaskList tasks={tasksList} todayDate={todayDate} todayRange={todayRange} changeTask={handleTaskStatueChange} variant="today" />
        </div>
    );
};

export default TodayView;