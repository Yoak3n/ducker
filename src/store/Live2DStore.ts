import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { Live2DState } from './types';


interface Live2DStoreState extends Live2DState {
    updateModelState: (state: Live2DState) => void;
}
const initialState: Live2DState = {
    show: true,
    modelPath: '/models/Mao/Mao.model3.json',
    width: 300,
    height: 480,
    position: {
        x: 20,
        y: -20,
    },
    scale: 0.06,
    loading: false,
    error: null,
};

export const useLive2DStore = create<Live2DStoreState>()(
    devtools(
        subscribeWithSelector(
            (set) => ({
                // 初始状态
                ...initialState,
                // 状态更新方法
                updateModelState: (state) => set(state),
            })
        ),
        {
            name: 'live2d-store',
        }
    )
);




