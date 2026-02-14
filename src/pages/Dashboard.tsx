import React, { useEffect, useMemo, useState } from 'react';

import { useI18n } from '@/hooks/use-i18n';
import { listen } from '@tauri-apps/api/event';
import '@/assets/Dashboard.css';
// import type { Task } from '@/types';
import type { WeekDay } from '@/types/modules/task';
import { useTaskStore } from '@/store';
import { extractTimeStampSecond } from '@/utils';
import TodayView from '@/components/Panel/Today';
import WeeklyView from '@/components/Panel/Weekly';
import MonthlyView from '@/components/Panel/Monthly';
import { Button } from '@/components/ui/button';

// 定义选项卡类型
type TabType = 'today' | 'weekly' | 'monthly';

// 定义星期类型
type DayOfWeek = WeekDay;

const TaskDashboard: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabType>('today');
  const { tasks, fetchTasks, toggleTaskCompletion } = useTaskStore();

  useEffect(() => {
    fetchTasks();
    let unlistenTaskChanged: Promise<() => void> | null = null;
    if (typeof window !== 'undefined') {
      try {
        unlistenTaskChanged = listen('task-changed', () => {
          fetchTasks();
        });
      } catch (error) {
        console.warn('事件监听器初始化失败:', error);
      }
    }
    return () => {
      if (unlistenTaskChanged) {
        unlistenTaskChanged.then(fn => fn()).catch(console.warn);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onToggleTask = async (taskId: string) => {
    await toggleTaskCompletion(taskId);
  };

  // 计算每周进度
  // const calculateWeeklyProgress = (weeklyTasks: Task[][]): number => {
  //   const totalTasks = weeklyTasks.reduce((acc, list) => acc + list.length, 0);
  //   const completedTasks = weeklyTasks.reduce((acc, list) => acc + list.filter(t => t.completed).length, 0);
  //   return totalTasks > 0 ? Math.round((completedTasks / totalTasks) * 100) : 0;
  // };

  // 计算每日进度（目前未使用，可按需恢复）


  const todayDate = useMemo(() => new Date(), []);
  const [currentMonthDate, setCurrentMonthDate] = useState<Date>(todayDate);
  const startOfDay = (d: Date) => new Date(d.getFullYear(), d.getMonth(), d.getDate(), 0, 0, 0, 0);
  const endOfDay = (d: Date) => new Date(d.getFullYear(), d.getMonth(), d.getDate(), 23, 59, 59, 999);
  const toSecond = (d: Date) => Math.floor(d.getTime() / 1000);

  const weekDays = useMemo(() => {
    const base = new Date();
    const day = base.getDay();
    const mondayOffset = ((day + 6) % 7); // 0 => Monday
    const monday = new Date(base);
    monday.setDate(base.getDate() - mondayOffset);
    const labels: { day: DayOfWeek; label: string }[] = [
      { day: 'monday', label: 'Mon' },
      { day: 'tuesday', label: 'Tue' },
      { day: 'wednesday', label: 'Wed' },
      { day: 'thursday', label: 'Thu' },
      { day: 'friday', label: 'Fri' },
      { day: 'saturday', label: 'Sat' },
      { day: 'sunday', label: 'Sun' },
    ];
    return labels.map((item, idx) => {
      const thisDate = new Date(monday);
      thisDate.setDate(monday.getDate() + idx);
      const start = toSecond(startOfDay(thisDate));
      const end = toSecond(endOfDay(thisDate));
      return { day: item.day, label: item.label, range: { start, end } };
    });
  }, []);

  // const tasksByRange = (range: { start: number; end: number }) =>
  //   tasks.filter(t => t.due_to && (() => {
  //     const ts = extractTimeStampSecond(t.due_to!);
  //     return ts >= range.start && ts <= range.end;
  //   })());

  // const weeklyBuckets = useMemo(() => weekDays.map(w => tasksByRange(w.range)), [tasks, weekDays]);
  // const weekProgress = calculateWeeklyProgress(weeklyBuckets);

  const startOfMonth = (d: Date) => new Date(d.getFullYear(), d.getMonth(), 1, 0, 0, 0, 0);
  const endOfMonth = (d: Date) => new Date(d.getFullYear(), d.getMonth() + 1, 0, 23, 59, 59, 999);

  const monthRange = useMemo(() => ({ 
      start: toSecond(startOfMonth(currentMonthDate)), 
      end: toSecond(endOfMonth(currentMonthDate)) 
    })
  , [currentMonthDate]);

  const monthlyTasks = useMemo(() => tasks.filter(t => t.due_to && (() => {
    const ts = extractTimeStampSecond(t.due_to!);
    return ts >= monthRange.start && ts <= monthRange.end;
  })()), [tasks, monthRange]);

  const monthlyCompleted = monthlyTasks.filter(t => t.completed).length;
  const monthlyTotal = monthlyTasks.length;
  const monthlyProgress = monthlyTotal > 0 ? (monthlyCompleted / monthlyTotal) * 100 : 0;

  const { t } = useI18n();

  return (
    <div className="task-dashboard">
      <div className="tabs flex justify-center mb-1 border-b border-gray-200">
        <button 
          className={activeTab === 'today' ? 'active' : ''}
          onClick={() => setActiveTab('today')}
        >
          {t("Daily")}
        </button>
        <button 
          className={activeTab === 'weekly' ? 'active' : ''}
          onClick={() => setActiveTab('weekly')}
        >
          {t("Weekly")}
        </button>
        <button 
          className={activeTab === 'monthly' ? 'active' : ''}
          onClick={() => setActiveTab('monthly')}
        >
          {t("Monthly")}
        </button>
      </div>
      
      <div className="tab-content bg-white p-4 rounded-lg shadow-md">
        {activeTab === 'today' && (
          <TodayView
            tasks={tasks}
            todayDate={todayDate}
            todayRange={{
              start: Math.floor(new Date(new Date().setHours(0, 0, 0, 0)).getTime() / 1000),
              end: Math.floor(new Date(new Date().setHours(23, 59, 59, 999)).getTime() / 1000),
            }}
            onToggleTask={onToggleTask}
          />
        )}
        {activeTab === 'weekly' && (
          <div>
            <h2>{t('Weekly')}</h2>
            {/* <div className="progress-bar">
              <div className="progress" style={{ width: `${weekProgress}%` }}></div>
              <span>{Math.round(weekProgress)}% 完成</span>
            </div> */}
            <WeeklyView tasks={tasks} weekDays={weekDays} todayDate={todayDate} onToggleTask={onToggleTask} />
          </div>
        )}
        {activeTab === 'monthly' && (
          <div>
            <div className="flex items-center justify-between">
              <h2>
                {t('Monthly')} ({currentMonthDate.toLocaleString('default', { year: 'numeric', month: 'long' })})
              </h2>
              <div className="flex items-center gap-2">
                <Button variant="outline" size="sm" onClick={() => setCurrentMonthDate(prev => new Date(prev.getFullYear(), prev.getMonth() - 1, 1))}>{t('Prev')}</Button>
                <Button variant="outline" size="sm" onClick={() => setCurrentMonthDate(new Date(todayDate.getFullYear(), todayDate.getMonth(), 1))}>{t('Today')}</Button>
                <Button variant="outline" size="sm" onClick={() => setCurrentMonthDate(prev => new Date(prev.getFullYear(), prev.getMonth() + 1, 1))}>{t('Next')}</Button>
              </div>
            </div>
            <div className="progress-bar">
              <div className="progress" style={{ width: `${monthlyProgress}%` }}></div>
              <span>
                {monthlyCompleted} / {monthlyTotal} 完成
              </span>
            </div>
            <MonthlyView tasks={tasks} monthDate={currentMonthDate} onToggleTask={onToggleTask} />
          </div>
        )}
      </div>
    </div>
  );
};

export default TaskDashboard;