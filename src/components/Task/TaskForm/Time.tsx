import {memo} from "react";
import DatetimePicker from "@/components/Date/DatetimePicker";
import type { TaskFormData } from "./type";
import type { Action, Period } from "@/types";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { reminderOptions } from "./options";
import { useI18n } from "@/hooks/use-i18n";
import { Clock } from "lucide-react";

interface Props {
    dueTo: string;
    reminderOffset: string;
    handleInputChange: (field: keyof TaskFormData, value: string | boolean | number | undefined | Action[] | Period) => void;
}

const Time = memo(function Time({ dueTo, reminderOffset, handleInputChange }: Props) {
    const {t} = useI18n();
    return (
        <div className="bg-card border border-border rounded-lg p-4 transition-[border-color,box-shadow] duration-200 ease-[cubic-bezier(0.23,1,0.32,1)]">
            <div className="flex items-center gap-2 mb-3 pb-2 border-b border-border bg-muted/50 rounded px-3 py-2 -mx-2">
                <Clock size={18} className="text-foreground" />
                <h3 className="m-0 text-sm font-semibold text-foreground">{t("Time Setting")}</h3>
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