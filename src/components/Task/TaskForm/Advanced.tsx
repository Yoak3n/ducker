import {memo, useState } from "react";

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { ActionSelect } from "@/components/Action";

import { useI18n } from "@/hooks/use-i18n";
import type { Action, Period } from "@/types";
import type { TaskFormData } from "./type.d";
import { ChevronUp, ChevronDown, Settings, CirclePlus, Brain, X, Check } from "lucide-react";


interface Props {
    actionsData: Action[];
    handleInputChange: (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => void;
}


const TaskFormAdvanced = memo(function TaskFormAdvanced({ actionsData, handleInputChange }: Props) {
    const [showAdvanced, setShowAdvanced] = useState(false);
    const [actionSelectOpen, setActionSelectOpen] = useState(false);
    const { t } = useI18n();

    return (
        <div className="mt-2 border-t border-border pt-4">
            <Collapsible>
                <CollapsibleTrigger asChild>
                    <button
                        type="button"
                        onClick={() => setShowAdvanced(!showAdvanced)}
                        className="flex items-center justify-between cursor-pointer w-full px-3 py-2 text-muted-foreground text-sm font-medium transition-all duration-200 bg-muted border border-border rounded-md hover:text-foreground hover:bg-muted hover:border-border"
                    >
                        <div className="flex items-center gap-2">
                            {showAdvanced ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
                            {t("Advanced")}
                        </div>
                    </button>
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <div className="bg-slate-50 border border-slate-200 rounded-lg p-3 mt-2">
                        <div className="flex justify-between items-center">
                            <div className="flex items-center gap-2">
                                <Settings size={18} className="text-slate-600" />
                                <h3 className="m-0 text-sm font-semibold text-slate-700">
                                    {t("Associated Action")}
                                </h3>
                                <span className="bg-blue-500 text-white text-xs font-semibold px-2 py-1 rounded-full min-w-5 text-center">{actionsData.length}</span>
                            </div>
                            <button
                                type="button"
                                className="flex items-center gap-2 px-3 py-2 bg-blue-500 text-white border-none rounded-md text-sm font-medium cursor-pointer transition-all duration-200 hover:bg-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)]"
                                onClick={() => setActionSelectOpen(true)}
                            >
                                <CirclePlus size={14} />
                                {t("Add Action")}
                            </button>
                        </div>

                        {actionsData.length === 0 && (
                            <div className="flex flex-col items-center py-6 px-3 text-center text-slate-600">
                                <Brain size={40} className="text-slate-300 mb-2" />
                                <p className="m-0 mb-2 text-sm font-medium text-slate-700">
                                    {t("No Associated Action")}
                                </p>
                                <span className="text-sm text-slate-500">
                                    {t("Click to add action")}
                                </span>
                            </div>
                        )}

                        {/* 显示已选择的 Actions */}
                        {actionsData.length > 0 && (
                            <div className="mt-3 p-3 bg-muted rounded-md border border-border">
                                <h4 className="m-0 mb-2 text-sm font-semibold text-foreground">
                                    {t("Selected Actions")} ({actionsData.length})
                                </h4>
                                <div className="flex flex-col gap-2">
                                    {actionsData.map((action, index) => (
                                        <div key={action.id} className="flex items-center gap-2 p-2 bg-card border border-border rounded text-sm">
                                            <span className="font-semibold text-muted-foreground min-w-5">{index + 1}.</span>
                                            <span className="font-medium text-foreground flex-1">{action.name}</span>
                                            <span className="text-muted-foreground text-xs bg-muted px-1 rounded">({action.type})</span>
                                            <button
                                                type="button"
                                                className="flex items-center justify-center w-5 h-5 border-none bg-red-500 text-white rounded cursor-pointer transition-colors duration-200 hover:bg-red-600"
                                                onClick={() => {
                                                    const newActions = actionsData.filter((_, i) => i !== index);
                                                    handleInputChange('actions', newActions);
                                                }}
                                            >
                                                <X size={12} />
                                            </button>
                                        </div>
                                    ))}
                                </div>
                            </div>
                        )}
                    </div>
                </CollapsibleContent>
            </Collapsible>

            {actionSelectOpen && (
                <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[9999] backdrop-blur-sm animate-[fadeIn_0.2s_ease-out]">
                    <div className="bg-card rounded-xl shadow-[0_25px_50px_-12px_rgba(0,0,0,0.25)] w-[90%] max-w-[800px] max-h-[80vh] flex flex-col overflow-hidden animate-[slideIn_0.3s_ease-out]">
                        <div className="flex items-center justify-between px-6 py-5 border-b border-border bg-gradient-to-r from-muted to-muted text-foreground">
                            <h2 className="m-0 text-xl font-semibold flex items-center gap-2 text-slate-700">
                                <Brain size={24} className="text-slate-600" />
                                {t("To Select Action")}
                            </h2>
                            <button
                                type="button"
                                className="flex items-center justify-center w-8 h-8 border-none bg-slate-600/10 text-slate-600 rounded-lg cursor-pointer transition-all duration-200 hover:bg-slate-600/20 hover:text-slate-700 hover:scale-105"
                                onClick={() => setActionSelectOpen(false)}
                            >
                                <X size={20} />
                            </button>
                        </div>
                        <div className="flex-1 p-6 overflow-y-auto min-h-0">
                            <ActionSelect
                                selectedActions={actionsData}
                                onActionsChange={(actions) => handleInputChange('actions', actions)}
                                multiSelect={true}
                                maxHeight='50vh'
                            />
                        </div>
                        <div className="flex justify-end gap-3 px-6 py-4 border-t border-border bg-muted">
                            <button
                                type="button"
                                className="px-5 py-2 border border-border bg-card text-muted-foreground rounded-lg text-sm font-medium cursor-pointer transition-all duration-200 hover:bg-muted hover:border-border hover:text-foreground"
                                onClick={() => setActionSelectOpen(false)}
                            >
                                {t("Cancel")}
                            </button>
                            <button
                                type="button"
                                className="px-5 py-2 border border-blue-500 bg-blue-500 text-white rounded-lg text-sm font-medium cursor-pointer transition-all duration-200 flex items-center gap-2 hover:bg-blue-600 hover:border-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)]"
                                onClick={() => setActionSelectOpen(false)}
                            >
                                <Check size={16} />
                                {t("Confirm")}
                            </button>
                        </div>
                    </div>
                </div>
            )}</div>
    );
});

export default TaskFormAdvanced;
