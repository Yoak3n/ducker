import { useMemo, useState, useCallback, useRef } from "react";
import type { Task } from "@/types";
import type { WeekDay } from "@/types/modules/task";
import { extractTimeStampSecond } from "@/utils";
import { ScrollArea } from "@/components/ui/scroll-area";
import TodayTaskItem from "../Today/TaskItem";

type WeeklyViewProps = {
  tasks: Task[];
  weekDays: { day: WeekDay; label: string; range: { start: number; end: number } }[];
  todayDate: Date;
  onToggleTask: (id: string) => void;
};

function ProgressRing({ percent }: { percent: number }) {
  return (
    <svg width="32" height="32" viewBox="0 0 36 36" className="week-progress-ring">
      <circle cx="18" cy="18" r="15.9" fill="none" stroke="#f1f5f9" strokeWidth="3" />
      <circle
        cx="18" cy="18" r="15.9" fill="none" stroke="#3498db" strokeWidth="3"
        strokeDasharray={`${percent} ${100 - percent}`}
        strokeDashoffset="25" strokeLinecap="round"
        style={{ transition: "stroke-dasharray 0.35s ease" }}
      />
    </svg>
  );
}

export default function WeeklyView({ tasks, weekDays, todayDate, onToggleTask }: WeeklyViewProps) {
  const todayTs = useMemo(() => {
    const d = new Date();
    d.setHours(0, 0, 0, 0);
    return d.getTime() / 1000;
  }, []);

  const [activeDay, setActiveDay] = useState<WeekDay | null>(null);
  const isAnimatingRef = useRef(false);
  const primaryRef = useRef<HTMLDivElement>(null);
  const sideCardRefs = useRef<Map<WeekDay, HTMLButtonElement>>(new Map());
  const containerRef = useRef<HTMLDivElement>(null);

  const setSideCardRef = useCallback((day: WeekDay, node: HTMLButtonElement | null) => {
    if (node) sideCardRefs.current.set(day, node);
    else sideCardRefs.current.delete(day);
  }, []);

  const getTasksByRange = useCallback(
    (range: { start: number; end: number }) =>
      tasks.filter((t) => {
        if (!t.due_to) return false;
        const ts = extractTimeStampSecond(t.due_to);
        return ts >= range.start && ts <= range.end;
      }),
    [tasks]
  );

  const primaryDay = useMemo<WeekDay>(() => {
    if (activeDay) return activeDay;
    return (
      weekDays.find(({ range }) => range.start <= todayTs && range.end >= todayTs)?.day ??
      weekDays[0]?.day
    );
  }, [activeDay, weekDays, todayTs]);

  const primaryEntry = weekDays.find((w) => w.day === primaryDay)!;
  const sideDays = weekDays.filter((w) => w.day !== primaryDay);

  const primaryTasks = getTasksByRange(primaryEntry.range);
  const completed = primaryTasks.filter((t) => t.completed).length;
  const total = primaryTasks.length;
  const percent = total > 0 ? Math.round((completed / total) * 100) : 0;
  const isToday = primaryEntry.range.start <= todayTs && primaryEntry.range.end >= todayTs;

  const sortedPrimaryTasks = useMemo(
    () =>
      [...primaryTasks].sort((a, b) => {
        if (!a.completed && b.completed) return -1;
        if (a.completed && !b.completed) return 1;
        return 0;
      }),
    [primaryTasks]
  );

  const handleSideDayClick = useCallback(
    (targetDay: WeekDay) => {
      if (isAnimatingRef.current) return;

      const primaryNode = primaryRef.current;
      const targetNode = sideCardRefs.current.get(targetDay);
      const containerNode = containerRef.current;
      if (!primaryNode || !targetNode || !containerNode) {
        setActiveDay(targetDay);
        return;
      }

      const DURATION = 420;
      const CONTAINER_RECT = containerNode.getBoundingClientRect();
      const primaryFrom = primaryNode.getBoundingClientRect();
      const targetFrom = targetNode.getBoundingClientRect();

      isAnimatingRef.current = true;

      // Primary → Target position
      const primaryDx = targetFrom.left - primaryFrom.left;
      const primaryDy = targetFrom.top - primaryFrom.top;
      const primaryScaleX = targetFrom.width / primaryFrom.width;
      const primaryScaleY = targetFrom.height / primaryFrom.height;

      // Target → Primary position
      const targetDx = primaryFrom.left - targetFrom.left;
      const targetDy = primaryFrom.top - targetFrom.top;
      const targetScaleX = primaryFrom.width / targetFrom.width;
      const targetScaleY = primaryFrom.height / targetFrom.height;

      // Capture all side card positions before state change
      const sidePositions = new Map<WeekDay, DOMRect>();
      sideCardRefs.current.forEach((node, day) => {
        if (day !== targetDay) sidePositions.set(day, node.getBoundingClientRect());
      });

      // Clone both cards as overlays
      const primaryClone = primaryNode.cloneNode(true) as HTMLElement;
      const targetClone = targetNode.cloneNode(true) as HTMLElement;

      const commonCloneStyle = {
        position: "absolute" as const,
        zIndex: "50" as const,
        pointerEvents: "none" as const,
        willChange: "transform, opacity" as const,
        transition: `transform ${DURATION}ms cubic-bezier(0.4, 0, 0.2, 1), opacity ${DURATION}ms cubic-bezier(0.4, 0, 0.2, 1)`,
      };

      // Position clones at their starting locations relative to container
      Object.assign(primaryClone.style, {
        ...commonCloneStyle,
        top: `${primaryFrom.top - CONTAINER_RECT.top}px`,
        left: `${primaryFrom.left - CONTAINER_RECT.left}px`,
        width: `${primaryFrom.width}px`,
        height: `${primaryFrom.height}px`,
        opacity: "1",
      });

      Object.assign(targetClone.style, {
        ...commonCloneStyle,
        top: `${targetFrom.top - CONTAINER_RECT.top}px`,
        left: `${targetFrom.left - CONTAINER_RECT.left}px`,
        width: `${targetFrom.width}px`,
        height: `${targetFrom.height}px`,
        opacity: "1",
      });

      containerNode.style.position = "relative";
      containerNode.appendChild(primaryClone);
      containerNode.appendChild(targetClone);

      // Hide originals
      primaryNode.style.opacity = "0";
      targetNode.style.opacity = "0";

      // Trigger reflow then animate
      primaryClone.getBoundingClientRect();

      requestAnimationFrame(() => {
        // Primary clone moves to target position and shrinks, then fades
        primaryClone.style.transform = `translate(${primaryDx}px, ${primaryDy}px) scale(${primaryScaleX}, ${primaryScaleY})`;
        primaryClone.style.opacity = "0";

        // Target clone moves to primary position and grows, then fades
        targetClone.style.transform = `translate(${targetDx}px, ${targetDy}px) scale(${targetScaleX}, ${targetScaleY})`;
        targetClone.style.opacity = "0";
      });

      // Update state so re-render creates the new layout
      setActiveDay(targetDay);

      // After re-render: measure new positions and animate side cards
      // Use setTimeout + rAF to ensure React has committed and browser has painted
      setTimeout(() => requestAnimationFrame(() => {
        // Animate remaining side cards from old to new positions
        const sideCardTransitions: HTMLElement[] = [];
        sideCardRefs.current.forEach((node, day) => {
          if (day === targetDay) return;
          const oldRect = sidePositions.get(day);
          if (!oldRect) return;
          const newRect = node.getBoundingClientRect();
          const dx = oldRect.left - newRect.left;
          const dy = oldRect.top - newRect.top;
          if (Math.abs(dx) < 0.5 && Math.abs(dy) < 0.5) return;

          node.style.transition = "none";
          node.style.transform = `translate(${dx}px, ${dy}px)`;
          node.style.zIndex = "40";
          sideCardTransitions.push(node);
        });

        // Force reflow for side card transforms
        if (sideCardTransitions.length > 0) {
          sideCardTransitions[0].getBoundingClientRect();
        }

        requestAnimationFrame(() => {
          sideCardTransitions.forEach((node) => {
            node.style.transition = `transform ${DURATION}ms cubic-bezier(0.4, 0, 0.2, 1)`;
            node.style.transform = "";
          });

          // Clean up after animation completes
          setTimeout(() => {
            sideCardTransitions.forEach((node) => {
              node.style.transition = "";
              node.style.transform = "";
              node.style.zIndex = "";
            });
          }, DURATION);
        });

        // Fade new primary and side card in
        const newPrimaryNode = primaryRef.current;
        const newTargetNode = sideCardRefs.current.get(primaryEntry.day);
        if (newPrimaryNode) {
          newPrimaryNode.style.opacity = "0";
          newPrimaryNode.style.transition = `opacity ${DURATION}ms cubic-bezier(0.4, 0, 0.2, 1)`;
          newPrimaryNode.style.opacity = "1";
        }
        if (newTargetNode) {
          newTargetNode.style.opacity = "0";
          newTargetNode.style.transition = `opacity ${DURATION}ms cubic-bezier(0.4, 0, 0.2, 1)`;
          newTargetNode.style.opacity = "1";
        }

        // Cleanup
        setTimeout(() => {
          primaryClone.remove();
          targetClone.remove();
          if (newPrimaryNode) {
            newPrimaryNode.style.opacity = "";
            newPrimaryNode.style.transition = "";
          }
          if (newTargetNode) {
            newTargetNode.style.opacity = "";
            newTargetNode.style.transition = "";
          }
          isAnimatingRef.current = false;
        }, DURATION);
      }), 0);
    },
    [primaryEntry.day]
  );

  return (
    <ScrollArea className="weekly-scroll-area">
      <div className="weekly-two-col" ref={containerRef}>
        {/* 左列：主卡片 */}
        <div className="week-primary" ref={primaryRef}>
          <div className="week-primary-header">
            <div className="week-primary-title-group">
              <span className={`week-primary-label ${isToday ? "week-primary-label--today" : ""}`}>
                {primaryEntry.label}
              </span>
              <span className="week-primary-sub">{isToday ? "今日任务" : "本周任务"}</span>
            </div>
            <div className="week-primary-ring">
              <ProgressRing percent={percent} />
              <span className="week-primary-pct">{percent}%</span>
            </div>
          </div>

          <div className="week-primary-stats">
            <span className="week-pstat">
              <strong>{total}</strong>
              <small>总计</small>
            </span>
            <span className="week-pstat-sep" />
            <span className="week-pstat">
              <strong className="week-pstat--done">{completed}</strong>
              <small>已完成</small>
            </span>
            <span className="week-pstat-sep" />
            <span className="week-pstat">
              <strong className="week-pstat--pending">{total - completed}</strong>
              <small>待完成</small>
            </span>
          </div>

          <div className="week-primary-body">
            {sortedPrimaryTasks.length > 0 ? (
              <ul className="task-list">
                {sortedPrimaryTasks.map((task) => (
                  <TodayTaskItem
                    key={task.id}
                    task={task}
                    changeTask={onToggleTask}
                    todayDate={todayDate}
                    today={
                      extractTimeStampSecond(task.due_to!) >= primaryEntry.range.start &&
                      extractTimeStampSecond(task.due_to!) <= primaryEntry.range.end
                    }
                    variant="weekly"
                  />
                ))}
              </ul>
            ) : (
              <div className="week-primary-empty">
                {isToday ? "今天没有任务" : "暂无任务"}
              </div>
            )}
          </div>
        </div>

        {/* 右列：2×3 紧凑卡片 */}
        <div className="week-side-grid">
          {sideDays.map(({ day, label, range }) => {
            const dayTasks = getTasksByRange(range);
            const dayCompleted = dayTasks.filter((t) => t.completed).length;
            const dayTotal = dayTasks.length;
            const dayPercent = dayTotal > 0 ? Math.round((dayCompleted / dayTotal) * 100) : 0;
            const sorted = [...dayTasks].sort((a, b) => {
              if (!a.completed && b.completed) return -1;
              if (a.completed && !b.completed) return 1;
              return 0;
            });

            return (
              <button
                key={day}
                type="button"
                className="week-side-card"
                ref={(node) => setSideCardRef(day, node)}
                onClick={() => handleSideDayClick(day)}
              >
                <div className="week-side-header">
                  <span className="week-side-label">{label}</span>
                  {dayTotal > 0 && (
                    <span className="week-side-count">
                      {dayCompleted}/{dayTotal}
                    </span>
                  )}
                </div>
                {dayTotal > 0 && (
                  <div className="week-side-bar">
                    <div className="week-side-bar-fill" style={{ width: `${dayPercent}%` }} />
                  </div>
                )}
                <div className="week-side-tasks">
                  {sorted.slice(0, 4).map((t) => (
                    <div key={t.id} className="week-side-row">
                      <span className={`week-side-dot ${t.completed ? "week-side-dot--done" : ""}`} />
                      <span className="week-side-name">{t.name}</span>
                    </div>
                  ))}
                  {sorted.length === 0 && <span className="week-side-empty">无任务</span>}
                </div>
              </button>
            );
          })}
        </div>
      </div>
    </ScrollArea>
  );
}
