import { useState, useEffect, type FC } from 'react';

import { Slider } from '@/components/ui/slider';
import { Button } from '../ui/button';

import { closeWindow } from '@/api';



interface Props {
    scale: number;
    updateScale: (scale: number) => void;
    position: { x: number, y: number };
    updatePosition: (position: { x: number, y: number }) => void;
    setIsSettingOpen: (isOpen: boolean) => void;
}

const closeMainWindow = async () => {
    await closeWindow('main')
}


const Setting: FC<Props> = ({ scale, updateScale, position, updatePosition, setIsSettingOpen }) => {
    const [modelScale, setModelScale] = useState(scale * 1000);
    const [modelPosition, setModelPosition] = useState(position);

    useEffect(() => {
        updateScale(modelScale / 1000);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [modelScale])
    useEffect(() => {
        updatePosition(modelPosition);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [modelPosition])

    return (
        <>
            <div className="flex flex-col gap-1 mx-3 py-1">
                <div className="flex col gap-3 mx-1">
                    <span>Scale</span>
                    <Slider
                        defaultValue={[modelScale]}
                        max={100}
                        min={10}
                        step={10}
                        onValueChange={(value) => setModelScale(value[0])}
                    />
                    <span>{modelScale / 100}</span>

                </div>
                <div className="flex col gap-3 mx-1">
                    <span>PosX</span>
                    <Slider
                        defaultValue={[modelPosition.x]}
                        max={100}
                        min={-100}
                        step={5}
                        onValueChange={(value) => setModelPosition({
                            x: value[0],
                            y: modelPosition.y,
                        })}
                    />
                    <span>{modelPosition.x}</span>

                </div>
                <div className="flex col gap-3 px-1">
                    <span>PosY</span>
                    <Slider
                        defaultValue={[position.y]}
                        max={100}
                        min={-100}
                        step={5}
                        onValueChange={(value) => setModelPosition({
                            x: modelPosition.x,
                            y: value[0],
                        })}
                    />
                    <span>{modelPosition.y}</span>
                </div>
            </div>

            <div className="flex justify-center w-full mt-2 mb-2">
                <Button className='w-1/2 h-6 cursor-pointer' onClick={() => {
                    setIsSettingOpen(false);
                    closeMainWindow()
                }}>关闭Live2D</Button>
            </div>
        </>
    )
}
export default Setting;