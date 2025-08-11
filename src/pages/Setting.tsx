import { useState,useEffect, type FC } from 'react';

import { cn } from "@/lib/utils"
import { Slider } from '@/components/ui/slider';
import { useLive2DStore } from '@/store/Live2DStore';


type SliderProps = React.ComponentProps<typeof Slider>

const Setting: FC<SliderProps> = ({ className, ...props }) => {
    const live2dStore = useLive2DStore();

    const [modelScale, setModelScale] = useState(60);
    useEffect(() => {
        
        live2dStore.updateModelState({
            ...live2dStore,
            scale: modelScale / 100,
        })
        console.log(live2dStore.scale);
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [modelScale])

    return (
        <>
            <Slider
                defaultValue={[modelScale]}
                max={100}
                step={1}
                onValueChange={(value) => setModelScale(value[0])}
                className={cn("w-[60%]", className)}
                {...props}
            />
        </>
    )
}
export default Setting;
