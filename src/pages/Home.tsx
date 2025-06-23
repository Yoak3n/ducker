import React from 'react';
import Live2DModelComponent from '@/components/Live2DModel';




const Home: React.FC = () => {
  return (
    <div className="home-page" >
      <div className="live2d-wrapper" data-tauri-drag-region>
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