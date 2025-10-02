import {memo} from "react";
import { Period, type Action } from "@/types";
import { periodOptions } from "./options";
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import type { TaskFormData } from "./type";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

import { useI18n } from "@/hooks/use-i18n";

interface Props {
    periodicInterval: Period;
    isPeriodic: boolean;
    setIsPeriodic: (v: boolean) => void;
    handleInputChange: (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => void;
}

const Periodic = memo(function Periodic({ periodicInterval, isPeriodic, setIsPeriodic, handleInputChange }: Props) {
    const { t } = useI18n();
    return (
        <div className="bg-gradient-to-br from-slate-50 to-slate-100 border border-slate-200 rounded-lg p-4 transition-all duration-200 hover:border-slate-300 hover:shadow-md">
            <div className="flex items-center gap-2 mb-3 pb-2 border-b border-gray-200 bg-white/80 rounded px-3 py-2 -mx-2">
                <span className="material-symbols-outlined text-purple-500 text-lg">repeat</span>
                <h3 className="m-0 text-sm font-semibold text-gray-800">{t("Period Setting")}</h3>
            </div>

            <div className="mb-3">
                <div className="flex flex-col gap-2">
                    <div className='flex flex-row items-center gap-2'>
                        <Checkbox
                            className='cursor-pointer'
                            checked={isPeriodic}
                            onCheckedChange={(v) => setIsPeriodic(v as boolean)}
                        />
                        <label className="font-medium text-gray-600 cursor-default">{t("Set as Periodic Task")}</label>
                    </div>
                    <span className="text-sm text-gray-500 italic ml-6">{t("Enable to repeat the task at the set interval")}</span>
                </div>
            </div>

            {isPeriodic && (
                <div className="mt-4 pt-4 border-t border-gray-200 animate-[slideDown_0.3s_ease-out] bg-white/50 rounded-md p-4">
                    <div className="mb-3">
                        <div className="mb-3">
                            <Label className='py-1'>{t("Periodic Interval")}</Label>
                            <Select
                                value={periodicInterval.toString()}
                                onValueChange={(value) => handleInputChange('periodicInterval', Number(value) as Period)}
                            >
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={t("Select Periodic Interval")} />
                                </SelectTrigger>
                                <SelectContent>
                                    {periodOptions.map(option => (
                                        <SelectItem key={option.value} value={option.value.toString()}>
                                            <div className="flex gap-1">
                                                <span className="font-medium text-gray-600">{t(option.label)}</span>
                                                <span className="text-sm text-gray-500">{t(option.description)}</span>
                                            </div>
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
});

export default Periodic;