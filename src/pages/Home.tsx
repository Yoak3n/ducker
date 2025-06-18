import React from 'react';
import Live2DModelComponent from '@/components/Live2DModel';




const Home: React.FC = () => {
  return (
    <div className="home-page">
      {/* <h1>欢迎来到Ducker应用</h1> */}
      <p>这是一个使用React Router实现的单页面应用</p>
      
      <div className="live2d-wrapper">
        <Live2DModelComponent 
          modelPath="/models/Mao/Mao.model3.json" 
          width={300} 
          height={500} 
          position={{ x: -150, y: 0 }} 
          scale={0.1} 
        />
      </div>
    </div>
  );
};

export default Home;