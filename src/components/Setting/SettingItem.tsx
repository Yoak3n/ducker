import { type FC } from "react";

interface SettingItemProps {
    id: string;
    title: string;
    children?: React.ReactNode;
}


const SettingItem: FC<SettingItemProps> = ({id,title,children}) => {
    return (<div className="setting-item" id={id}>
        <h2 className="setting-item-title">{title}</h2>
        {children}
    </div>)
}

export default SettingItem;