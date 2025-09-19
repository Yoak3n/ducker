import { type FC, useCallback, useEffect, useRef, useState } from 'react';

import * as PIXI from 'pixi.js';
// Import specifically from cubism4 module for model3.json files
import { Live2DModel, InternalModel } from 'pixi-live2d-display/cubism4';
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuTrigger,
} from "@/components/ui/context-menu"
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerTitle
} from "@/components/ui/drawer"

import ContextItems from "./ContextItems"
import Model from './Model';


import './index.css';
import { useLive2DStore } from '@/store/Live2DStore';

// import { Slider } from '../ui/slider';
import Setting from './Setting';


// 注册Live2D模型加载器
// 将PIXI声明为全局变量
window.PIXI = PIXI;

let app: PIXI.Application | null = null;
// 确保Live2DModel可以使用PIXI.Ticker
Live2DModel.registerTicker(PIXI.Ticker);



const Live2DModelComponent: FC = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const appRef = useRef<PIXI.Application | null>(null);
  const modelRef = useRef<Live2DModel<InternalModel>>(null);
  const live2dStore = useLive2DStore();
  const [isSettingOpen, setIsSettingOpen] = useState(false);

  useEffect(() => {
    if (modelRef.current) {
      modelRef.current.scale.set(live2dStore.scale);
      modelRef.current.x = live2dStore.position.x;
      modelRef.current.y = live2dStore.position.y;

    }
  }, [live2dStore.scale, live2dStore.position])

  const updateModelScale = (scale: number) => {
    live2dStore.updateModelState({
      ...live2dStore,
      scale: scale,
    })
  }
  const updateModelPosition = (position: { x: number, y: number }) => {
    live2dStore.updateModelState({
      ...live2dStore,
      position,
    })
  }


  const loadModel = useCallback(async () => {
    try {
      // 加载模型
      const model = await Live2DModel.from(live2dStore.modelPath, {
        autoUpdate: true,
        // autoLoad: true
      });
      // 设置模型属性
      model.scale.set(live2dStore.scale);
      model.x = live2dStore.position.x;
      model.y = live2dStore.position.y;

      // 确保app.stage存在
      if (appRef.current && appRef.current.stage) {
        // 添加模型到舞台
        appRef.current.stage.addChild(model);
        modelRef.current = model;
        // 启用交互
        model.interactive = false;
        model.buttonMode = false;

        // 添加点击事件 - 随机动作
        model.on('pointerdown', () => {
          console.log('Model clicked');
          try {
            // 尝试播放随机动作
            const motionManager = model.internalModel?.motionManager;
            if (motionManager && motionManager.definitions) {
              const motions = motionManager.definitions;
              const groups = Object.keys(motions);
              if (groups.length > 0) {
                const groupName = groups[Math.floor(Math.random() * groups.length)];
                const group = motions[groupName];
                if (group && group.length > 0) {
                  const index = Math.floor(Math.random() * group.length);
                  model.motion(groupName, index);
                }
              }
            }
          } catch (e) {
            console.error('Error playing motion:', e);
          }
        });
      } else {
        console.error('PIXI application stage is not available');
      }
    } catch (error) {
      console.error('Failed to load Live2D model:', error);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!canvasRef.current) return;

    // 创建PIXI应用
    app = new window.PIXI.Application({
      view: canvasRef.current,
      width: live2dStore.width,
      height: live2dStore.height,
      backgroundAlpha: 0.0,
      // antialias: true,
      // resizeTo:window,
      autoStart: true
    });

    appRef.current = app;
    // 将PIXI应用添加到DOM
    // containerRef.current.appendChild(app.view as unknown as Node);
    appRef.current = app;

    // 加载Live2D模型
    loadModel();
    console.log(appRef.current);

    // 清理函数
    return () => {
      if (modelRef.current) {
        modelRef.current.destroy();
        modelRef.current = null;
      }
      if (appRef.current) {
        appRef.current.destroy(true, true);
        appRef.current = null;
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      <Drawer open={isSettingOpen} onClose={() => setIsSettingOpen(false)}>
        <DrawerContent>
          <div className="mx-auto w-full max-w-sm">
            {/* <DrawerHeader> */}
              <DrawerTitle></DrawerTitle>
              <DrawerDescription></DrawerDescription>
            {/* </DrawerHeader> */}
            <Setting 
            scale={live2dStore.scale} 
            updateScale={updateModelScale} 
            position={live2dStore.position} 
            updatePosition={updateModelPosition} 
            setIsSettingOpen={setIsSettingOpen}
            />
          </div>
        </DrawerContent>
      </Drawer>
      <ContextMenu >
        <ContextMenuTrigger>
          <Model
            container={containerRef}
            canvas={canvasRef}
            width={live2dStore.width}
            height={live2dStore.height}
          />
        </ContextMenuTrigger>
        <ContextMenuContent >
          <ContextItems isSettingOpen={isSettingOpen} setIsSettingOpen={setIsSettingOpen} />
        </ContextMenuContent>
      </ContextMenu>
    </>

  );
};

export default Live2DModelComponent;