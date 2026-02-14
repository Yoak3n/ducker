import type { Task } from "@/types";
import type { WeekDay } from "@/types/modules/task";
import { extractTimeStampSecond } from "@/utils";
import TaskList from "../Today/TaskList";

type WeeklyViewProps = {
  tasks: Task[];
  weekDays: { day: WeekDay; label: string; range: { start: number; end: number } }[];
  todayDate: Date;
  onToggleTask: (id: string) => void;
};

export default function WeeklyView({ tasks, weekDays, todayDate, onToggleTask }: WeeklyViewProps) {
  const getTasksByRange = (range: { start: number; end: number }) =>
    tasks.filter(t => {
      if (!t.due_to) return false;
      const ts = extractTimeStampSecond(t.due_to);
      return ts >= range.start && ts <= range.end;
    });

  return (
    <div className="weekly-view custom-scrollbar">
      <div className="week-days">
        {weekDays.map(({ day, label, range }) => {
          const dayTasks = getTasksByRange(range);
          const completed = dayTasks.filter(t => t.completed).length;
          const total = dayTasks.length;
          const percent = total > 0 ? Math.round((completed / total) * 100) : 0;
          return (
            <div key={day} className="day-column">
              <h3>{label}</h3>
              <div className="day-content">
                <div className="day-progress">
                  <small>{percent}% 完成</small>
                  <div className="mini-progress">
                    <div style={{ width: `${percent}%` }}></div>
                  </div>
                </div>
                <TaskList
                  tasks={dayTasks}
                  todayDate={todayDate}
                  todayRange={range}
                  changeTask={onToggleTask}
                  variant="weekly"
                />
                {total === 0 && <ul className="task-list"><li className="empty">无任务</li></ul>}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}


