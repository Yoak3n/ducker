import React from 'react';
import Live2DModelComponent from '../components/Live2DModel';

const Home: React.FC = () => {
  return (
    <div className="home-page">
      {/* <h1>欢迎来到Ducker应用</h1> */}
      <p>这是一个使用React Router实现的单页面应用</p>
      
      <div className="live2d-wrapper">
        <Live2DModelComponent 
          modelPath="/models/Haru/Haru.model3.json" 
          width={300} 
          height={500} 
          position={{ x: -150, y: 0 }} 
          scale={0.3} 
        />
      </div>
      
      <div className="features">
        <div className="feature">
          <h2>React</h2>
          <p>使用最新的React特性构建用户界面</p>
        </div>
        <div className="feature">
          <h2>TypeScript</h2>
          <p>类型安全的JavaScript超集</p>
        </div>
        <div className="feature">
          <h2>Vite</h2>
          <p>快速的前端构建工具</p>
        </div>
      </div>
    </div>
  );
};

export default Home;