import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

import { actionTypes } from "@/types";
import { useActionStore } from "@/store";
import ActionCard from "../ActionCard";


const ActionsList = () => {
    const [searchTerm, setSearchTerm] = useState("");
    const [selectedType, setSelectedType] = useState("all");
    const [control, setControl] = useState(false);
    const navigate = useNavigate()
    const actionStore = useActionStore()


    useEffect(() => {
        const loadActions = async () => {
            try {
                await actionStore.fetchActions();
            } catch (error) {
                console.error('加载动作失败:', error);
            }
        };
        loadActions();
        // eslint-disable-next-line react-hooks/exhaustive-deps
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
                    actionStore.actions.map(action => (
                        <ActionCard
                            key={action.id}
                            action={action}
                            selectable
                            onSelect={() => navigate(`/action/modify/${action.id}`)}

                        />
                    ))
                }
            </div>

        </div>
    );
}


export default ActionsList