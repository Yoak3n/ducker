import { useEffect, useState } from "react";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Clock, Play, CheckCircle2, Circle } from "lucide-react";

import { useI18n } from "@/hooks/use-i18n";
import { get_all_startup_periodic_tasks } from "@/api/modules/task";
import { Period, type PeriodicTask } from "@/types";
import { formatDateOnly } from "@/utils/date";

interface StartupTasksSheetProps {
  children: React.ReactNode;
}

const StartupTasksSheet = ({ children }: StartupTasksSheetProps) => {
  const { t } = useI18n();
  const [startupTasks, setStartupTasks] = useState<PeriodicTask[]>([]);
  const [loading, setLoading] = useState(false);
  const [isOpen, setIsOpen] = useState(false);

  // 获取启动时任务
  const fetchStartupTasks = async () => {
    setLoading(true);
    try {
      const startupTasks = await get_all_startup_periodic_tasks();
      console.log(startupTasks);
      setStartupTasks(startupTasks);
    } catch (error) {
      console.error("获取启动时任务失败:", error);
    } finally {
      setLoading(false);
    }
  };

  // 当弹窗打开时获取数据
  useEffect(() => {
    if (isOpen) {
      fetchStartupTasks();
    }
  }, [isOpen]);

  // 获取任务类型标签
  const getTaskTypeBadge = (interval: number) => {
    if (interval === Period.OnStart) {
      return (
        <Badge variant="secondary" className="flex items-center gap-1">
          <Play className="w-3 h-3" />
          {t("Period OnStart")}
        </Badge>
      );
    } else if (interval === Period.OnceStarted) {
      return (
        <Badge variant="outline" className="flex items-center gap-1">
          <Clock className="w-3 h-3" />
          {t("Period OnceStarted")}
        </Badge>
      );
    }
    return null;
  };

  // 格式化时间
  const formatTime = (timestamp?: number) => {
    if (!timestamp) return t("Never");
    return new Date(timestamp * 1000).toLocaleString();
  };

  return (
    <Sheet open={isOpen} onOpenChange={setIsOpen}>
      <SheetTrigger asChild>
        {children}
      </SheetTrigger>
      <SheetContent className="w-[400px] sm:w-[540px]">
        <SheetHeader >
          <SheetTitle className="flex items-center gap-2">
            <Play className="w-5 h-5" />
            {t("Startup Tasks")}
          </SheetTitle>
          <SheetDescription>
            {t("View and manage tasks that run on application startup")}
          </SheetDescription>
        </SheetHeader>

        <div className="mt-6">
          {loading ? (
            <div className="flex items-center justify-center py-8">
              <div className="text-sm text-muted-foreground">
                {t("Loading")}...
              </div>
            </div>
          ) : startupTasks.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-8 text-center">
              <Clock className="w-12 h-12 text-muted-foreground mb-4" />
              <h3 className="text-lg font-medium mb-2">
                {t("No Startup Tasks")}
              </h3>
              <p className="text-sm text-muted-foreground">
                {t("No startup tasks have been configured yet")}
              </p>
            </div>
          ) : (
            <ScrollArea className="h-[calc(100vh-200px)]">
              <div className="space-y-4">
                {startupTasks.map((periodicTask, index) => (
                  <div key={periodicTask.id || index}>
                    <div className="flex items-start space-x-3 p-4 rounded-lg border bg-card">
                      <div className="flex-shrink-0 mt-1">
                        {periodicTask.task.completed ? (
                          <CheckCircle2 className="w-5 h-5 text-green-500" />
                        ) : (
                          <Circle className="w-5 h-5 text-muted-foreground" />
                        )}
                      </div>
                      
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center justify-between mb-2">
                          <h4 className="text-sm font-medium truncate">
                            {periodicTask.task.name}
                          </h4>
                          {getTaskTypeBadge(periodicTask.interval)}
                        </div>
                        
                        <div className="space-y-2 text-xs text-muted-foreground">
                          <div className="flex items-center justify-between">
                            <span>{t("Task Value")}:</span>
                            <span className="font-medium">
                              {periodicTask.task.value || 0}
                            </span>
                          </div>
                          
                          {periodicTask.last_period && (
                            <div className="flex items-center justify-between">
                              <span>{t("Last Run")}:</span>
                              <span>{formatTime(periodicTask.last_period)}</span>
                            </div>
                          )}
                          
                          <div className="flex items-center justify-between">
                            <span>{t("Created")}:</span>
                            <span>
                                {formatDateOnly(periodicTask.task.created_at)}
                            </span>
                          </div>
                          
                          {periodicTask.task.actions && periodicTask.task.actions.length > 0 && (
                            <div className="flex items-center justify-between">
                              <span>{t("Actions")}:</span>
                              <span>{periodicTask.task.actions.length} {t("actions")}</span>
                            </div>
                          )}
                        </div>
                      </div>
                    </div>
                    
                    {index < startupTasks.length - 1 && (
                      <Separator className="my-4" />
                    )}
                  </div>
                ))}
              </div>
            </ScrollArea>
          )}
        </div>

      </SheetContent>
    </Sheet>
  );
};

export default StartupTasksSheet;