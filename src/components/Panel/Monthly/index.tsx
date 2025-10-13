import type { Task } from "@/types";
import { extractTimeStampSecond, formatDateOnly } from "@/utils";
import { Card } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";

type MonthlyViewProps = {
  tasks: Task[];
  monthDate: Date; // any date within the month to display
  onToggleTask: (id: string) => void;
};

function startOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), 0, 0, 0, 0);
}

function endOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 999);
}

function getMonthMatrix(target: Date) {
  const firstDay = new Date(target.getFullYear(), target.getMonth(), 1);
  const lastDay = new Date(target.getFullYear(), target.getMonth() + 1, 0);
  // Monday as first column, map JS getDay (0=Sun) to (1..7), where 1=Mon, 7=Sun
  const firstWeekday = ((firstDay.getDay() + 6) % 7) + 1; // 1..7
  const daysInMonth = lastDay.getDate();

  const cells: Date[] = [];
  // Leading blanks from previous month
  const leading = firstWeekday - 1; // 0..6
  for (let i = 0; i < leading; i++) {
    const d = new Date(firstDay);
    d.setDate(firstDay.getDate() - (leading - i));
    cells.push(d);
  }
  // Current month days
  for (let d = 1; d <= daysInMonth; d++) {
    cells.push(new Date(target.getFullYear(), target.getMonth(), d));
  }
  // Trailing blanks to complete the last week
  const remainder = cells.length % 7;
  if (remainder !== 0) {
    const need = 7 - remainder;
    const base = new Date(lastDay);
    for (let i = 1; i <= need; i++) {
      const d = new Date(base);
      d.setDate(base.getDate() + i);
      cells.push(d);
    }
  }
  return cells;
}

export default function MonthlyView({ tasks, monthDate, onToggleTask }: MonthlyViewProps) {
  const todayStr = formatDateOnly(new Date().toISOString());
  const monthCells = getMonthMatrix(monthDate);

  const getTasksForDate = (date: Date) => {
    const start = Math.floor(startOfDay(date).getTime() / 1000);
    const end = Math.floor(endOfDay(date).getTime() / 1000);
    return tasks.filter(t => {
      if (!t.due_to) return false;
      const ts = extractTimeStampSecond(t.due_to);
      return ts >= start && ts <= end;
    });
  };

  const isSameMonth = (date: Date) => date.getMonth() === monthDate.getMonth();
  const isWeekend = (date: Date) => {
    const d = date.getDay();
    return d === 0 || d === 6; // Sun or Sat
  };

  const weekLabels = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

  return (
    <div className="monthly-grid">
      <div className="grid grid-cols-7 gap-2 mb-2 text-sm text-muted-foreground">
        {weekLabels.map(w => (
          <div key={w} className="text-center select-none">{w}</div>
        ))}
      </div>
      <ScrollArea className="h-[60vh] w-full">
        <div className="grid grid-cols-7 gap-2 pr-2">
          {monthCells.map((date) => {
            const dateStr = formatDateOnly(date.toISOString());
            const dayTasks = getTasksForDate(date);
            const isToday = dateStr === todayStr;
            const weekend = isWeekend(date);
            return (
              <Card
                key={date.toISOString()}
                className={`min-h-28 p-2 transition-all duration-150 outline-none hover:shadow-sm ${
                  isSameMonth(date) ? '' : 'opacity-40'
                } ${isToday ? 'bg-gradient-to-br from-primary/5 to-transparent ring-inset ring-1 ring-primary border-primary/60' : ''} ${weekend ? 'bg-muted/20' : ''}`}
              >
                <div className="flex items-center justify-between mb-1">
                  <span className={`text-xs font-medium select-none ${weekend ? 'text-muted-foreground' : ''}`}>{date.getDate()}</span>
                  {dayTasks.length > 0 && (
                    <Badge variant="outline" className="h-5 px-1.5 py-0 text-[10px] text-primary border-primary/30">
                      {dayTasks.length}
                    </Badge>
                  )}
                </div>
                <ScrollArea className="pr-1" style={{ height: '6rem' }}>
                  <ul className="space-y-1">
                    {dayTasks.map(t => (
                      <li key={t.id} className="group flex items-center gap-2 text-xs">
                        <input
                          className="size-3.5 accent-primary cursor-pointer"
                          type="checkbox"
                          checked={t.completed}
                          onChange={() => onToggleTask(t.id)}
                        />
                        <TooltipProvider disableHoverableContent>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <span className={`truncate transition-colors ${t.completed ? 'line-through text-muted-foreground/70' : 'group-hover:text-foreground'}`}>{t.name}</span>
                            </TooltipTrigger>
                            <TooltipContent side="top" className="max-w-64">
                              <span className="break-words text-xs">{t.name}</span>
                            </TooltipContent>
                          </Tooltip>
                        </TooltipProvider>
                      </li>
                    ))}
                    {dayTasks.length === 0 && (
                      <li className="text-[11px] text-muted-foreground/60">—</li>
                    )}
                  </ul>
                </ScrollArea>
              </Card>
            );
          })}
        </div>
      </ScrollArea>
    </div>
  );
}


