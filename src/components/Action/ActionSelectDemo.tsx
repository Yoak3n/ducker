import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import ActionSelect from './ActionSelect';
import type{ Action } from '@/types/modules/action';
import { exec_action } from '@/utils/action';

export default function ActionSelectDemo() {
  const [selectedActions, setSelectedActions] = useState<Action[]>([]);
  const [isMultiSelect, setIsMultiSelect] = useState(true);
  const [isExecuting, setIsExecuting] = useState(false);

  const handleExecuteChain = async () => {
    if (selectedActions.length === 0) return;
    
    setIsExecuting(true);
    
    try {
      for (let i = 0; i < selectedActions.length; i++) {
        const action = selectedActions[i];
        console.log(`执行第 ${i + 1} 个 Action: ${action.name}`);
        
        // 执行action
        exec_action(action);
        
        // 如果有等待时间，则等待
        if (action.wait > 0) {
          await new Promise(resolve => setTimeout(resolve, action.wait));
        }
      }
      
      console.log('Action 调用链执行完成!');
    } catch (error) {
      console.error('执行 Action 链时出错:', error);
    } finally {
      setIsExecuting(false);
    }
  };

  const getTotalWaitTime = () => {
    return selectedActions.reduce((total, action) => total + action.wait, 0);
  };

  return (
    <div className="action-select-demo p-6 max-w-6xl mx-auto space-y-6">
      {/* 标题和说明 */}
      <div className="text-center space-y-2">
        <h1 className="text-2xl font-bold flex items-center justify-center gap-2">
          <span className="material-symbols-outlined text-3xl">
            settings_applications
          </span>
          Action 选择器演示
        </h1>
        <p className="text-gray-600">
          支持搜索过滤、类型筛选、多选链式调用的 Action 管理组件
        </p>
      </div>

      {/* 控制面板 */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span className="material-symbols-outlined">
              tune
            </span>
            控制面板
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center gap-4">
            <Button
              variant={isMultiSelect ? "default" : "outline"}
              onClick={() => {
                setIsMultiSelect(!isMultiSelect);
                if (!isMultiSelect) {
                  // 切换到单选模式时，只保留第一个选中的action
                  setSelectedActions(selectedActions.slice(0, 1));
                }
              }}
              className="flex items-center gap-2"
            >
              <span className="material-symbols-outlined text-sm">
                {isMultiSelect ? 'check_box' : 'check_box_outline_blank'}
              </span>
              多选模式
            </Button>
            
            <Button
              variant="outline"
              onClick={() => setSelectedActions([])}
              disabled={selectedActions.length === 0}
              className="flex items-center gap-2"
            >
              <span className="material-symbols-outlined text-sm">
                clear_all
              </span>
              清空选择
            </Button>
          </div>

          {/* 执行控制 */}
          {selectedActions.length > 0 && (
            <div className="space-y-3">
              <Separator />
              
              <div className="flex items-center justify-between">
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <Badge variant="secondary">
                      已选择 {selectedActions.length} 个 Action
                    </Badge>
                    {getTotalWaitTime() > 0 && (
                      <Badge variant="outline">
                        总等待时间: {getTotalWaitTime()}ms
                      </Badge>
                    )}
                  </div>
                  <p className="text-xs text-gray-500">
                    {isMultiSelect ? '将按顺序执行所有选中的 Actions' : '执行选中的 Action'}
                  </p>
                </div>
                
                <Button
                  onClick={handleExecuteChain}
                  disabled={isExecuting}
                  className="flex items-center gap-2"
                >
                  {isExecuting ? (
                    <>
                      <span className="material-symbols-outlined text-sm animate-spin">
                        refresh
                      </span>
                      执行中...
                    </>
                  ) : (
                    <>
                      <span className="material-symbols-outlined text-sm">
                        play_arrow
                      </span>
                      执行 {isMultiSelect ? 'Action 链' : 'Action'}
                    </>
                  )}
                </Button>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Action 选择器 */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span className="material-symbols-outlined">
              list
            </span>
            Action 选择器
          </CardTitle>
        </CardHeader>
        <CardContent>
          <ActionSelect
            selectedActions={selectedActions}
            onActionsChange={setSelectedActions}
            multiSelect={isMultiSelect}
            maxHeight="500px"
          />
        </CardContent>
      </Card>

      {/* 功能特性说明 */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span className="material-symbols-outlined">
              info
            </span>
            功能特性
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <h4 className="font-medium flex items-center gap-2">
                <span className="material-symbols-outlined text-sm">
                  search
                </span>
                搜索过滤
              </h4>
              <p className="text-sm text-gray-600">
                支持按 Action 名称和描述进行实时搜索过滤
              </p>
            </div>
            
            <div className="space-y-2">
              <h4 className="font-medium flex items-center gap-2">
                <span className="material-symbols-outlined text-sm">
                  filter_list
                </span>
                类型筛选
              </h4>
              <p className="text-sm text-gray-600">
                快速筛选文件操作、目录操作、命令执行等不同类型的 Actions
              </p>
            </div>
            
            <div className="space-y-2">
              <h4 className="font-medium flex items-center gap-2">
                <span className="material-symbols-outlined text-sm">
                  link
                </span>
                链式调用
              </h4>
              <p className="text-sm text-gray-600">
                支持多选模式，可按顺序组合多个 Actions 形成调用链
              </p>
            </div>
            
            <div className="space-y-2">
              <h4 className="font-medium flex items-center gap-2">
                <span className="material-symbols-outlined text-sm">
                  visibility
                </span>
                信息展示
              </h4>
              <p className="text-sm text-gray-600">
                ActionCard 直观展示 Action 的类型、描述、命令和参数信息
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}