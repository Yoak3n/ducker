import {memo, useState } from "react";

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { ActionSelect } from "@/components/Action";

import { useI18n } from "@/hooks/use-i18n";
import type { Action, Period } from "@/types";
import type { TaskFormData } from "./type.d";


interface Props {
    actionsData: Action[];
    handleInputChange: (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => void;
}


const TaskFormAdvanced = memo(function TaskFormAdvanced({ actionsData, handleInputChange }: Props) {
    const [showAdvanced, setShowAdvanced] = useState(false);
    const [actionSelectOpen, setActionSelectOpen] = useState(false);
    const { t } = useI18n();

    return (
        <div className="mt-2 border-t border-gray-200 pt-4">
            <Collapsible>
                <CollapsibleTrigger onClick={() => setShowAdvanced(!showAdvanced)}>
                    <div className="flex items-center justify-between cursor-pointer w-full px-3 py-2 text-gray-500 text-sm font-medium transition-all duration-200 bg-slate-50 border border-slate-200 rounded-md hover:text-gray-600 hover:bg-slate-100 hover:border-slate-300">
                        <div className="flex items-center gap-2">
                            <span className="material-symbols-outlined">
                                {showAdvanced ? 'expand_less' : 'expand_more'}
                            </span>
                            {t("Advanced")}
                        </div>
                    </div>
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <div className="bg-slate-50 border border-slate-200 rounded-lg p-3 mt-2">
                        <div className="flex justify-between items-center">
                            <div className="flex items-center gap-2">
                                <span className="material-symbols-outlined text-slate-600 text-lg">settings</span>
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
                                <span className="material-symbols-outlined text-sm">add_circle</span>
                                {t("Add Action")}
                            </button>
                        </div>

                        {actionsData.length === 0 && (
                            <div className="flex flex-col items-center py-6 px-3 text-center text-slate-600">
                                <span className="material-symbols-outlined text-4xl text-slate-300 mb-2">psychology_alt</span>
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
                            <div className="mt-3 p-3 bg-gray-50 rounded-md border border-gray-200">
                                <h4 className="m-0 mb-2 text-sm font-semibold text-gray-700">
                                    {t("Selected Actions")} ({actionsData.length})
                                </h4>
                                <div className="flex flex-col gap-2">
                                    {actionsData.map((action, index) => (
                                        <div key={action.id} className="flex items-center gap-2 p-2 bg-white border border-gray-200 rounded text-sm">
                                            <span className="font-semibold text-gray-500 min-w-5">{index + 1}.</span>
                                            <span className="font-medium text-gray-800 flex-1">{action.name}</span>
                                            <span className="text-gray-500 text-xs bg-gray-200 px-1 rounded">({action.type})</span>
                                            <button
                                                type="button"
                                                className="flex items-center justify-center w-5 h-5 border-none bg-red-500 text-white rounded cursor-pointer transition-colors duration-200 hover:bg-red-600"
                                                onClick={() => {
                                                    const newActions = actionsData.filter((_, i) => i !== index);
                                                    handleInputChange('actions', newActions);
                                                }}
                                            >
                                                <span className="material-symbols-outlined text-xs">close</span>
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
                    <div className="bg-white rounded-xl shadow-[0_25px_50px_-12px_rgba(0,0,0,0.25)] w-[90%] max-w-[800px] max-h-[80vh] flex flex-col overflow-hidden animate-[slideIn_0.3s_ease-out]">
                        <div className="flex items-center justify-between px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-slate-50 to-gray-100 text-slate-800">
                            <h2 className="m-0 text-xl font-semibold flex items-center gap-2 text-slate-700">
                                <span className="material-symbols-outlined text-2xl text-slate-600">psychology_alt</span>
                                {t("To Select Action")}
                            </h2>
                            <button
                                className="flex items-center justify-center w-8 h-8 border-none bg-slate-600/10 text-slate-600 rounded-lg cursor-pointer transition-all duration-200 hover:bg-slate-600/20 hover:text-slate-700 hover:scale-105"
                                onClick={() => setActionSelectOpen(false)}
                            >
                                <span className="material-symbols-outlined text-xl">close</span>
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
                        <div className="flex justify-end gap-3 px-6 py-4 border-t border-gray-200 bg-gray-50">
                            <button
                                className="px-5 py-2 border border-gray-300 bg-white text-gray-500 rounded-lg text-sm font-medium cursor-pointer transition-all duration-200 hover:bg-gray-50 hover:border-gray-400 hover:text-gray-600"
                                onClick={() => setActionSelectOpen(false)}
                            >
                                {t("Cancel")}
                            </button>
                            <button
                                className="px-5 py-2 border border-blue-500 bg-blue-500 text-white rounded-lg text-sm font-medium cursor-pointer transition-all duration-200 flex items-center gap-2 hover:bg-blue-600 hover:border-blue-600 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(59,130,246,0.3)]"
                                onClick={() => setActionSelectOpen(false)}
                            >
                                <span className="material-symbols-outlined text-base">check</span>
                                {t("Confirm")}
                            </button>
                        </div>
                    </div>
                </div>
            )}</div>
    );
});

export default TaskFormAdvanced;