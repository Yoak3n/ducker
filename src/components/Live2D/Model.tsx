import type { FC, Ref } from 'react';

interface Props {
    container: Ref<HTMLDivElement | null>,
    canvas: Ref<HTMLCanvasElement | null>,
    width: number,
    height: number,
}

const Live2DModel: FC<Props> = ({ container, canvas, width, height }) => {
    return (
        <div ref={container} className="live2d-container" style={{ width: width, height: height }}>
            <canvas
                ref={canvas}
                data-tauri-drag-region
            />
        </div>
    )
}
export default Live2DModel;

