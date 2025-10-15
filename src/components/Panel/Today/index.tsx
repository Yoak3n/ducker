import { useMemo } from "react";
import { Button } from "@/components/ui/button";
import type { Task } from "@/types";
import { extractTimeStampSecond } from "@/utils";
import { showWindow } from "@/api";
import { useI18n } from "@/hooks/use-i18n";
import TodayTaskList from "./TaskList";
import StartupTasksSheet from "./StartupTasksSheet";
import "./index.css"



type TodayViewProps = {
    tasks: Task[];
    todayDate: Date;
    todayRange: { start: number; end: number };
    onToggleTask: (id: string) => void;
};

const TodayView = ({ tasks, todayDate, todayRange, onToggleTask }: TodayViewProps) => {
    const { t } = useI18n();

    const tasksList = useMemo(() => {
        return tasks
            .filter(task => {
                const isToday = (extractTimeStampSecond(task.due_to!) >= todayRange.start && extractTimeStampSecond(task.due_to!) <= todayRange.end);
                const isOverdue = extractTimeStampSecond(task.due_to!) <= todayDate.getTime() / 1000 && !task.completed;
                return isToday || isOverdue;
            });
    }, [tasks, todayDate, todayRange]);


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

    const handleTaskStatueChange = (id: string) => onToggleTask(id)

    return (
        <div className="today-view overflow-y-auto">
            <div className="today-title flex justify-between items-center">
                <h2>{t("Today")} {todayDate.toLocaleDateString()}</h2>
                <div className="flex gap-2">
                    <StartupTasksSheet>
                        <Button variant="ghost" className="cursor-pointer">
                            {t("Startup Tasks")}
                        </Button>
                    </StartupTasksSheet>
                    <Button variant="outline" className="cursor-pointer" onClick={() => handleCreateTask()}>
                        {t("Create Task")}
                    </Button>
                </div>
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
            <TodayTaskList  tasks={tasksList} todayDate={todayDate} todayRange={todayRange} changeTask={handleTaskStatueChange} variant="today" />
        </div>
    );
};

export default TodayView;