import { useMemo } from "react";
import type { Task } from "@/types";
import { extractTimeStampSecond } from "@/utils";
import { ScrollArea } from "@/components/ui/scroll-area";

type MonthlyViewProps = {
  tasks: Task[];
  monthDate: Date;
  onToggleTask: (id: string) => void;
};

function startOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), 0, 0, 0, 0);
}

function endOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 999);
}

function getMonthMatrix(target: Date) {
  const firstDay = new Date(target.getFullYear(), target.getMonth(), 1);
  const lastDay = new Date(target.getFullYear(), target.getMonth() + 1, 0);
  const firstWeekday = ((firstDay.getDay() + 6) % 7) + 1;
  const daysInMonth = lastDay.getDate();

  const cells: Date[] = [];
  const leading = firstWeekday - 1;
  for (let i = 0; i < leading; i++) {
    const d = new Date(firstDay);
    d.setDate(firstDay.getDate() - (leading - i));
    cells.push(d);
  }
  for (let d = 1; d <= daysInMonth; d++) {
    cells.push(new Date(target.getFullYear(), target.getMonth(), d));
  }
  const remainder = cells.length % 7;
  if (remainder !== 0) {
    const need = 7 - remainder;
    const base = new Date(lastDay);
    for (let i = 1; i <= need; i++) {
      const d = new Date(base);
      d.setDate(base.getDate() + i);
      cells.push(d);
    }
  }
  return cells;
}

function getMonthLabel(date: Date) {
  const months = ["一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"];
  return months[date.getMonth()];
}

export default function MonthlyView({ tasks, monthDate, onToggleTask }: MonthlyViewProps) {
  const today = useMemo(() => {
    const d = new Date();
    return { year: d.getFullYear(), month: d.getMonth(), date: d.getDate() };
  }, []);

  const monthCells = useMemo(() => getMonthMatrix(monthDate), [monthDate]);

  const isSameMonth = (date: Date) => date.getMonth() === monthDate.getMonth();
  const isWeekend = (date: Date) => {
    const d = date.getDay();
    return d === 0 || d === 6;
  };
  const isToday = (date: Date) =>
    date.getFullYear() === today.year &&
    date.getMonth() === today.month &&
    date.getDate() === today.date;

  const getTasksForDate = (date: Date) => {
    const start = Math.floor(startOfDay(date).getTime() / 1000);
    const end = Math.floor(endOfDay(date).getTime() / 1000);
    return tasks.filter((t) => {
      if (!t.due_to) return false;
      const ts = extractTimeStampSecond(t.due_to);
      return ts >= start && ts <= end;
    });
  };

  const weekLabels = [
    { label: "一", weekend: false },
    { label: "二", weekend: false },
    { label: "三", weekend: false },
    { label: "四", weekend: false },
    { label: "五", weekend: false },
    { label: "六", weekend: true },
    { label: "日", weekend: true },
  ];

  const monthStats = useMemo(() => {
    const monthStart = new Date(monthDate.getFullYear(), monthDate.getMonth(), 1);
    const monthEnd = new Date(monthDate.getFullYear(), monthDate.getMonth() + 1, 0, 23, 59, 59, 999);
    const startTs = Math.floor(monthStart.getTime() / 1000);
    const endTs = Math.floor(monthEnd.getTime() / 1000);
    const monthTasks = tasks.filter((t) => {
      if (!t.due_to) return false;
      const ts = extractTimeStampSecond(t.due_to);
      return ts >= startTs && ts <= endTs;
    });
    const total = monthTasks.length;
    const done = monthTasks.filter((t) => t.completed).length;
    return { total, done, pending: total - done };
  }, [tasks, monthDate]);

  return (
    <ScrollArea className="monthly-scroll-area">
      <div className="monthly-container">
        <div className="monthly-header">
          <div className="monthly-title-group">
            <span className="monthly-title">{getMonthLabel(monthDate)}</span>
            <span className="monthly-year">{monthDate.getFullYear()}</span>
          </div>
          <div className="monthly-stats">
            <span className="monthly-stat">
              <span className="monthly-stat-dot monthly-stat-dot--total" />
              <strong>{monthStats.total}</strong> 总计
            </span>
            <span className="monthly-stat">
              <span className="monthly-stat-dot monthly-stat-dot--done" />
              <strong>{monthStats.done}</strong> 已完成
            </span>
            <span className="monthly-stat">
              <span className="monthly-stat-dot monthly-stat-dot--pending" />
              <strong>{monthStats.pending}</strong> 待完成
            </span>
          </div>
        </div>

        <div className="monthly-weekdays">
          {weekLabels.map(({ label, weekend }) => (
            <div key={label} className={`monthly-weekday ${weekend ? "monthly-weekday--weekend" : ""}`}>
              {label}
            </div>
          ))}
        </div>

        <div className="monthly-grid">
          {monthCells.map((date) => {
            const dayTasks = getTasksForDate(date);
            const outside = !isSameMonth(date);
            const weekend = isWeekend(date);
            const todayCell = isToday(date);
            const sorted = [...dayTasks].sort((a, b) => {
              if (!a.completed && b.completed) return -1;
              if (a.completed && !b.completed) return 1;
              return 0;
            });

            return (
              <div
                key={date.toISOString()}
                className={`monthly-cell${outside ? " monthly-cell--outside" : ""}${weekend ? " monthly-cell--weekend" : ""}${todayCell ? " monthly-cell--today" : ""}`}
              >
                <div className="monthly-cell-header">
                  <span className="monthly-cell-num">{date.getDate()}</span>
                  {dayTasks.length > 0 && (
                    <span className="monthly-cell-count">{dayTasks.length}</span>
                  )}
                </div>
                <div className="monthly-cell-tasks">
                  {sorted.length > 0 ? (
                    sorted.map((t) => (
                      <div key={t.id} className="monthly-task-row">
                        <input
                          type="checkbox"
                          className="monthly-task-check"
                          checked={t.completed}
                          onChange={() => onToggleTask(t.id)}
                        />
                        <span className={`monthly-task-name ${t.completed ? "monthly-task-name--done" : ""}`}>
                          {t.name}
                        </span>
                      </div>
                    ))
                  ) : (
                    <span className="monthly-cell-empty">-</span>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </ScrollArea>
  );
}
