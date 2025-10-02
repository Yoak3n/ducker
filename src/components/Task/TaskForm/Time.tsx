import {memo} from "react";
import DatetimePicker from "@/components/Date/DatetimePicker";
import type { TaskFormData } from "./type";
import type { Action, Period } from "@/types";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { reminderOptions } from "./options";
import { useI18n } from "@/hooks/use-i18n";

interface Props {
    dueTo: string;
    reminderOffset: string;
    handleInputChange: (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => void;
}

const Time = memo(function Time({ dueTo, reminderOffset, handleInputChange }: Props) {
    const {t} = useI18n();
    return (
        <div className="bg-gradient-to-br from-purple-50 to-purple-100 border border-purple-200 rounded-lg p-4 transition-all duration-200 hover:border-purple-300 hover:shadow-md">
            <div className="flex items-center gap-2 mb-3 pb-2 border-b border-gray-200 bg-white/80 rounded px-3 py-2 -mx-2">
                <span className="material-symbols-outlined text-purple-600 text-lg">schedule</span>
                <h3 className="m-0 text-sm font-semibold text-gray-800">{t("Time Setting")}</h3>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div className="mb-3">
                    <DatetimePicker
                        datetime={dueTo}
                        setDatetime={(datetime) => handleInputChange('due_to', datetime)}
                    />
                </div>
                <div className="mb-3">
                    <Label className='py-1'>{t("Reminder Setting")}</Label>
                    <Select
                        value={reminderOffset}
                        onValueChange={(value) => handleInputChange('reminderOffset', value)}
                    >
                        <SelectTrigger className="w-full">
                            <SelectValue placeholder={t("Select")+ t("Reminder Setting")} />
                        </SelectTrigger>
                        <SelectContent>
                            {reminderOptions.map(option => (
                                <SelectItem key={option.value} value={option.value}>
                                    {t(option.label)}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </div>
            </div>
        </div>
    );
});

export default Time;