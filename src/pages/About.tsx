import React from 'react';

const About: React.FC = () => {
  return (
    <div className="about-page">
      <h1>关于我们</h1>
      <p>Ducker是一个示例React单页面应用，展示了如何使用React Router进行路由管理。</p>
      <div className="about-content">
        <h2>项目特点</h2>
        <ul>
          <li>使用React Router进行路由管理</li>
          <li>TypeScript类型支持</li>
          <li>组件化开发</li>
          <li>响应式设计</li>
        </ul>
        <h2>技术栈</h2>
        <ul>
          <li>React</li>
          <li>TypeScript</li>
          <li>React Router</li>
          <li>Vite</li>
        </ul>
      </div>
    </div>
  );
};

export default About;