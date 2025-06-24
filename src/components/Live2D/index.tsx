import React, { useCallback, useEffect, useRef, useState } from 'react';
import * as PIXI from 'pixi.js';
// Import specifically from cubism4 module for model3.json files
import { Live2DModel, InternalModel } from 'pixi-live2d-display/cubism4';
import ContextMenu from './ContextMenu';

// 注册Live2D模型加载器
// 将PIXI声明为全局变量
window.PIXI = PIXI;
let app: PIXI.Application | null = null;
// 确保Live2DModel可以使用PIXI.Ticker
// Live2DModel.registerTicker(PIXI.Ticker);

interface Live2DModelProps {
  modelPath: string;
  width?: number;
  height?: number;
  position?: { x: number; y: number };
  scale?: number;
}

// 自定义右键菜单项接口
interface MenuItem {
  label: string;
  action: () => void;
}

const Live2DModelComponent: React.FC<Live2DModelProps> = ({
  modelPath,
  width = 300,
  height = 500,
  position = { x: 0, y: 0 },
  scale = 0.3,
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const appRef = useRef<PIXI.Application | null>(null);
  const modelRef = useRef<Live2DModel<InternalModel>>(null);
  
  // 右键菜单状态
  const [showMenu, setShowMenu] = useState(false);
  const [menuPosition, setMenuPosition] = useState({ x: 0, y: 0 });
  const loadModel = useCallback(async () => {
    try {
      console.log('Loading model from path:', modelPath);

      // 加载模型
      const model = await Live2DModel.from(modelPath, {
        autoUpdate: true,
        // autoLoad: true
      });

      console.log('Model loaded successfully:', model);

      // 设置模型属性
      model.scale.set(scale);
      model.x = position.x;
      model.y = position.y;

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
  }, [modelPath, position, scale]);
  // 菜单项定义
  const menuItems: MenuItem[] = [
    {
      label: '重新加载模型',
      action: () => {
        if (appRef.current && modelRef.current) {
          // 移除当前模型
          appRef.current.stage.removeChild(modelRef.current);
          modelRef.current.destroy();
          modelRef.current = null;
          
          // 重新加载模型
          loadModel();
        }
      }
    },
    {
      label: '隐藏模型',
      action: () => {
        if (modelRef.current) {
          modelRef.current.visible = !modelRef.current.visible;
        }
      }
    },
    {
      label: '关闭菜单',
      action: () => setShowMenu(false)
    }
  ];

  // 处理右键点击事件
  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault(); // 阻止默认右键菜单
    setMenuPosition({ x: e.clientX, y: e.clientY });
    setShowMenu(true);
  };
  
  // 处理点击其他区域关闭菜单
  const handleClickOutside = () => {
    if (showMenu) {
      setShowMenu(false);
    }
  };

  useEffect(() => {
    // 添加全局点击事件监听器来关闭菜单
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  }, [handleClickOutside, showMenu]);

  useEffect(() => {
    if (!canvasRef.current) return;
    
    // 创建PIXI应用
    app = new PIXI.Application({
      view: canvasRef.current,
      width,
      height,
      backgroundAlpha: 0,
      // antialias: true,
      // resizeTo:window,
      autoStart: true
    });

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
  }, [modelPath, width, height, position, scale, loadModel]);
  
  // 加载模型函数


  return (
    <div ref={containerRef} className="live2d-container">
      <canvas 
        ref={canvasRef} 
        data-tauri-drag-region 
        onContextMenu={handleContextMenu} 
      />
      
      {/* 自定义右键菜单 */}
      {showMenu && (
        <ContextMenu
          menuItems={menuItems}
          position={menuPosition}
          hideMenuCallback={setShowMenu}
        />
      )
      }
    </div>
  );
};

export default Live2DModelComponent;