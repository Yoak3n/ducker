import { type FC } from "react";

interface SettingItemProps {
    id: string;
    title: string;
    children?: React.ReactNode;
}


const SettingItem: FC<SettingItemProps> = ({ id, title, children }) => {
    return (<div className="setting-item" id={id}>
        <div className="setting-item-title-wrapper">        
            <h2 className="setting-item-title">{title}</h2>
        </div>

        {children}
    </div>)
}

export default SettingItem;