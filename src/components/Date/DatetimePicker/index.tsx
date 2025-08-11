import { useEffect, useState } from "react"

import { ChevronDownIcon } from "lucide-react"

import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import { formatDatetime } from "@/utils"


interface Props {
    datetime: string | undefined,
    setDatetime: (datetime: string | undefined) => void
}

export default function DatetimePicker({ datetime, setDatetime }: Props) {
    const [open, setOpen] = useState(false)
    const [date, setDate] = useState<Date | undefined>(undefined)
    const [time, setTime] = useState<string>("00:00:00")

    useEffect(() => {
        if (datetime) {
            const [date, time] = datetime.split(" ")
            setDate(new Date(date))
            setTime(time)
        } else {
            setDate(undefined)
            setTime("00:00:00")
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    useEffect(() => {
        if (date && time) {
            const datetime = new Date(date)
            const [hour, minute, second] = time.split(":")
            datetime.setHours(Number(hour), Number(minute), Number(second))
            //   YYYY-MM-DD HH:mm:ss
            setDatetime(formatDatetime(datetime))

        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [date, time])
    return (
        <div>
            <Label className="px-1 py-1">
                截止时间
            </Label>
            <div className="flex gap-3">
                <Popover open={open} onOpenChange={setOpen}>
                    <PopoverTrigger asChild >
                        <Button
                            variant="outline"
                            id="date-picker"
                            className="w-40 justify-between font-normal"
                        >
                            {date ? date.toLocaleDateString() : "选择日期"}
                            <ChevronDownIcon />
                        </Button>
                    </PopoverTrigger>
                    <PopoverContent className="w-auto overflow-hidden p-0" align="start">
                        <Calendar
                            mode="single"
                            selected={date}
                            captionLayout="dropdown"
                            onSelect={(date) => {
                                setDate(date)
                                setOpen(false)
                            }}
                        />
                    </PopoverContent>

                </Popover>
                <Input
                    type="time"
                    id="time-picker"
                    step="1"
                    value={time}
                    onChange={(e) => {
                        e.preventDefault()
                        setTime(e.target.value)
                    }}
                    className="w-1/3 bg-background appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none"
                />
            </div>

        </div>
    )
}
