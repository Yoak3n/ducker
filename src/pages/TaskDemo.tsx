import React from 'react';
import { TaskModalExample } from '@/components/Task';

/**
 * TaskModal组件演示页面
 * 展示任务创建/编辑浮动窗口的功能
 */
export default function TaskDemo() {
  return (
    <div className="task-demo-page">
      <div style={{ 
        maxWidth: '1200px', 
        margin: '0 auto', 
        padding: '20px',
        minHeight: '100vh'
      }}>
        <header style={{ 
          marginBottom: '32px',
          textAlign: 'center',
          borderBottom: '1px solid #e5e7eb',
          paddingBottom: '20px'
        }}>
          <h1 style={{ 
            color: '#1f2937',
            fontSize: '2rem',
            fontWeight: '700',
            marginBottom: '8px'
          }}>
            任务管理演示
          </h1>
          <p style={{ 
            color: '#6b7280',
            fontSize: '1.125rem'
          }}>
            体验类似滴答清单的任务创建和编辑功能
          </p>
        </header>

        <main>
          <div style={{
            background: 'white',
            borderRadius: '12px',
            boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
            padding: '24px'
          }}>
            <TaskModalExample />
          </div>
        </main>

        <footer style={{
          marginTop: '40px',
          textAlign: 'center',
          color: '#6b7280',
          fontSize: '0.875rem'
        }}>
          <div style={{
            background: '#f9fafb',
            padding: '20px',
            borderRadius: '8px',
            border: '1px solid #e5e7eb'
          }}>
            <h3 style={{ marginBottom: '12px', color: '#374151' }}>功能特性</h3>
            <div style={{ 
              display: 'grid', 
              gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
              gap: '16px',
              textAlign: 'left'
            }}>
              <div>
                <h4 style={{ color: '#1f2937', marginBottom: '8px' }}>基础功能</h4>
                <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                  <li>✅ 创建新任务</li>
                  <li>✅ 编辑现有任务</li>
                  <li>✅ 创建子任务</li>
                  <li>✅ 设置截止时间</li>
                </ul>
              </div>
              <div>
                <h4 style={{ color: '#1f2937', marginBottom: '8px' }}>高级功能</h4>
                <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                  <li>✅ 关联动作配置</li>
                  <li>✅ 自动执行设置</li>
                  <li>✅ 提醒时间设置</li>
                  <li>✅ 多种动作类型</li>
                </ul>
              </div>
              <div>
                <h4 style={{ color: '#1f2937', marginBottom: '8px' }}>界面特性</h4>
                <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                  <li>✅ 响应式设计</li>
                  <li>✅ 平滑动画</li>
                  <li>✅ 现代化UI</li>
                  <li>✅ 键盘快捷键</li>
                </ul>
              </div>
            </div>
          </div>
        </footer>
      </div>
    </div>
  );
}