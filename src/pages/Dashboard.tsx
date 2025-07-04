import React, { useState } from 'react';
import '@/assets/Dashboard.css';
import type {TaskMocks,  WeeklyTasks } from '@/types';
import {allData} from '@/mocks/task';

import TodayView from '@/components/Panel/Today';


// 定义选项卡类型
type TabType = 'today' | 'weekly' | 'monthly';

// 定义星期类型
type DayOfWeek = keyof WeeklyTasks;

const TaskDashboard: React.FC = () => {
  // 当前选中的选项卡
  const [activeTab, setActiveTab] = useState<TabType>('today');
  
  // 模拟任务数据，带有类型注解
  const [tasks, setTasks] = useState<TaskMocks>(allData);

  
  const toggleTaskCompletion = (period: TabType, day: DayOfWeek | null, taskId: string) => {
    setTasks(prevTasks => {
      
      const newTasks: TaskMocks = {...prevTasks};
      
      if (period === 'today' || period === 'monthly') {
        newTasks[period] = newTasks[period].map(task => 
          task.id === taskId ? {...task, completed: !task.completed} : task
        );
      } else if (period === 'weekly' && day) {
        newTasks.weekly = {...newTasks.weekly};
        newTasks.weekly[day] = newTasks.weekly[day].map(task => 
          task.id === taskId ? {...task, completed: !task.completed} : task
        );
      }
      
      return newTasks;
    });
  };

  // 计算每周进度
  const calculateWeeklyProgress = (): number => {
    const days: DayOfWeek[] = ['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday'];
    let totalTasks = 0;
    let completedTasks = 0;
    
    days.forEach(day => {
      tasks.weekly[day].forEach(task => {
        totalTasks++;
        if (task.completed) completedTasks++;
      });
    });
    
    return totalTasks > 0 ? Math.round((completedTasks / totalTasks) * 100) : 0;
  };

  // 计算每日进度
  const calculateDailyProgress = (day: DayOfWeek): number => {
    const dayTasks = tasks.weekly[day];
    if (dayTasks.length === 0) return 0;
    
    const completed = dayTasks.filter(task => task.completed).length;
    return Math.round((completed / dayTasks.length) * 100);
  };


  // 渲染周视图
  const renderWeeklyView = () => {
    const weekProgress = calculateWeeklyProgress();
    const days: DayOfWeek[] = ['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday'];

    return (
      <div className="weekly-view">
        <h2>本周任务</h2>
        <div className="progress-bar">
          <div 
            className="progress" 
            style={{ width: `${weekProgress}%` }}
          ></div>
          <span>{weekProgress}% 完成</span>
        </div>
        
        <div className="week-days">
          {days.map(day => (
            <div key={day} className="day-column">
              <h3>{day.charAt(0).toUpperCase() + day.slice(1)}</h3>
              <div className="day-progress">
                <small>{calculateDailyProgress(day)}% 完成</small>
                <div className="mini-progress">
                  <div style={{ width: `${calculateDailyProgress(day)}%` }}></div>
                </div>
              </div>
              <ul className="task-list">
                {tasks.weekly[day].map(task => (
                  <li key={task.id} className={task.completed ? 'completed' : ''}>
                    <input
                      type="checkbox"
                      checked={task.completed}
                      onChange={() => toggleTaskCompletion('weekly', day, task.id)}
                    />
                    <span>{task.name}</span>
                  </li>
                ))}
                {tasks.weekly[day].length === 0 && <li className="empty">无任务</li>}
              </ul>
            </div>
          ))}
        </div>
      </div>
    );
  };

  // 渲染月视图
  const renderMonthlyView = () => {
    const completedCount = tasks.monthly.filter(t => t.completed).length;
    const totalCount = tasks.monthly.length;
    const progressPercent = totalCount > 0 ? (completedCount / totalCount) * 100 : 0;

    return (
      <div className="monthly-view">
        <h2>月度任务 ({new Date().toLocaleString('default', { month: 'long' })})</h2>
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
          {tasks.monthly.map(task => (
            <li key={task.id} className={task.completed ? 'completed' : ''}>
              <input
                type="checkbox"
                checked={task.completed}
                onChange={() => toggleTaskCompletion('monthly', null, task.id)}
              />
              <span>{task.name}</span>
            </li>
          ))}
        </ul>
      </div>
    );
  };

  return (
    <div className="task-dashboard">
      <div className="tabs">
        <button 
          className={activeTab === 'today' ? 'active' : ''}
          onClick={() => setActiveTab('today')}
        >
          今日任务
        </button>
        <button 
          className={activeTab === 'weekly' ? 'active' : ''}
          onClick={() => setActiveTab('weekly')}
        >
          本周任务
        </button>
        <button 
          className={activeTab === 'monthly' ? 'active' : ''}
          onClick={() => setActiveTab('monthly')}
        >
          月度任务
        </button>
      </div>
      
      <div className="tab-content">
        {activeTab === 'today' && <TodayView />}
        {activeTab === 'weekly' && renderWeeklyView()}
        {activeTab === 'monthly' && renderMonthlyView()}
      </div>
    </div>
  );
};

export default TaskDashboard;