import { type FC } from 'react';
import Live2DModelComponent from '@/components/Live2D';


const Live2D: FC = () => {

  return (
    <>
      <Live2DModelComponent />
    </>
  )

}


const Home: FC = () => {
  return (
    <div className="home-page" >
      <div className="live2d-wrapper" data-tauri-drag-region>
        <Live2D />
        {/* <Live2DModelComponent 
          modelPath={state.modelPath} 
          width={state.width} 
          height={state.height} 
          position={state.position} 
          scale={state.scale}
        /> */}
      </div>
    </div>
  );
};

export default Home;