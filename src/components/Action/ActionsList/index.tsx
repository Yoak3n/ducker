import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import {
    ContextMenu,
    ContextMenuTrigger,
    ContextMenuContent,
    ContextMenuItem,
} from '@/components/ui/context-menu'
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

import { actionTypes } from "@/types";
import { useActionStore } from "@/store";
import ActionCard from "../ActionCard";
import { toast } from "sonner";


const ActionsList = () => {
    const [searchTerm, setSearchTerm] = useState("");
    const [selectedType, setSelectedType] = useState("all");
    // const [control, setControl] = useState(false);
    const navigate = useNavigate()
    const fetchActions = useActionStore(state=>state.fetchActions)
    const executeSingleAction = useActionStore(state=>state.executeSingleAction)
    const deleteAction = useActionStore(state=>state.deleteAction)
    const actions = useActionStore(state=>state.actions)
    const loadActions = async () => {
        try {
            await fetchActions();
        } catch (error) {
            console.error('加载动作失败:', error);
        }
    };

    useEffect(() => {
        loadActions();
    }, [])


    return (
        <div className="flex w-full flex-col gap-6">
            <div className="flex w-full justify-between">
                <div className="relative w-full pr-5">
                    <span className="material-symbols-outlined absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400">
                        search
                    </span>
                    <Input
                        placeholder="搜索 actions..."
                        value={searchTerm}
                        onChange={(e) => setSearchTerm(e.target.value)}
                        className="pl-10"
                    />
                </div>
                <Button variant="outline" size="icon" onClick={(e) => {
                    e.preventDefault();
                    navigate("/action/modify/add");
                    // loadActions();
                }}>
                    <span className="material-symbols-outlined">
                        add
                    </span>
                </Button>
            </div>
            <div className="flex flex-wrap gap-2">
                {actionTypes.map(type => (
                    <Button
                        key={type.value}
                        variant={selectedType === type.value ? "default" : "outline"}
                        size="sm"
                        onClick={() => setSelectedType(type.value)}
                        className="text-xs"
                    >
                        <span className="material-symbols-outlined text-sm mr-1">
                            {type.icon}
                        </span>
                        {type.label}
                    </Button>
                ))}
            </div>
            <div className="grid grid-cols-3 gap-4">
                {
                    actions.filter(action => {
                        if (selectedType === "all") {
                            return action.name.toLowerCase().includes(searchTerm.toLowerCase())
                        }
                        return action.type === selectedType && action.name.toLowerCase().includes(searchTerm.toLowerCase())
                    }).map(action => (
                        <ContextMenu key={action.id}>
                            <ContextMenuTrigger>
                                <ActionCard
                                    key={action.id}
                                    action={action}
                                    selectable
                                    onSelect={() => navigate(`/action/modify/${action.id}`)}
                                />
                            </ContextMenuTrigger>
                            <ContextMenuContent>
                                <ContextMenuItem onClick={
                                    async () => {
                                        const result = await executeSingleAction(action)
                                        if (result.success) {
                                            toast.success('动作执行成功');
                                        } else {
                                            toast.error(`动作执行失败: ${result.error}`);
                                        }
                                    }
                                    }>
                                    运行
                                </ContextMenuItem>
                                <ContextMenuItem className="text-red-500" onClick={() => deleteAction(action.id)}>
                                    删除
                                </ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenu>

                    ))
                }
            </div>

        </div>
    );
}


export default ActionsList
