import { useState, useMemo } from 'react';
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import ActionCard from "../ActionCard";
import type{ Action } from "@/types/modules/action";
import { mockActions, actionTypes } from "@/mocks/action";

interface ActionSelectProps {
  selectedActions?: Action[];
  onActionsChange?: (actions: Action[]) => void;
  multiSelect?: boolean;
  maxHeight?: string;
}

export default function ActionSelect({ 
  selectedActions = [], 
  onActionsChange,
  multiSelect = false,
  maxHeight = "400px"
}: ActionSelectProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedType, setSelectedType] = useState('all');
  const [internalSelectedActions, setInternalSelectedActions] = useState<Action[]>(selectedActions);

  // 使用内部状态或外部传入的状态
  const currentSelectedActions = onActionsChange ? selectedActions : internalSelectedActions;

  // 过滤actions
  const filteredActions = useMemo(() => {
    return mockActions.filter(action => {
      const matchesSearch = action.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           action.description.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesType = selectedType === 'all' || action.type === selectedType;
      return matchesSearch && matchesType;
    });
  }, [searchTerm, selectedType]);

  // 处理action选择
  const handleActionSelect = (action: Action) => {
    if (!multiSelect) {
      // 单选模式
      const newSelection = currentSelectedActions.includes(action) ? [] : [action];
      if (onActionsChange) {
        onActionsChange(newSelection);
      } else {
        setInternalSelectedActions(newSelection);
      }
      return;
    }

    // 多选模式
    const isSelected = currentSelectedActions.some(selected => selected.id === action.id);
    let newSelection: Action[];
    
    if (isSelected) {
      // 取消选择
      newSelection = currentSelectedActions.filter(selected => selected.id !== action.id);
    } else {
      // 添加选择
      newSelection = [...currentSelectedActions, action];
    }
    
    if (onActionsChange) {
      onActionsChange(newSelection);
    } else {
      setInternalSelectedActions(newSelection);
    }
  };

  // 处理移除选中的action
  const handleActionRemove = (action: Action) => {
    const newSelection = currentSelectedActions.filter(selected => selected.id !== action.id);
    if (onActionsChange) {
      onActionsChange(newSelection);
    } else {
      setInternalSelectedActions(newSelection);
    }
  };

  // 清空所有选择
  const handleClearAll = () => {
    if (onActionsChange) {
      onActionsChange([]);
    } else {
      setInternalSelectedActions([]);
    }
  };

  // 获取action的选择顺序
  const getSelectionOrder = (action: Action): number | undefined => {
    const index = currentSelectedActions.findIndex(selected => selected.id === action.id);
    return index >= 0 ? index + 1 : undefined;
  };

  return (
    <div className="action-select-container space-y-4">
      {/* 搜索框 */}
      <div className="relative">
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

      {/* 类型过滤器 */}
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

      {/* 已选择的actions链 */}
      {multiSelect && currentSelectedActions.length > 0 && (
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h4 className="text-sm font-medium flex items-center gap-2">
              <span className="material-symbols-outlined text-sm">
                link
              </span>
              Action 调用链 ({currentSelectedActions.length})
            </h4>
            <Button 
              variant="ghost" 
              size="sm" 
              onClick={handleClearAll}
              className="text-xs text-red-500 hover:text-red-700"
            >
              清空全部
            </Button>
          </div>
          
          <div className="flex flex-wrap gap-2 p-3 bg-gray-50 rounded-lg">
            {currentSelectedActions.map((action, index) => (
              <div key={action.id} className="flex items-center gap-1">
                <Badge variant="secondary" className="text-xs">
                  {index + 1}. {action.name}
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => handleActionRemove(action)}
                    className="ml-1 h-4 w-4 p-0 hover:bg-red-100"
                  >
                    <span className="material-symbols-outlined text-xs text-red-500">
                      close
                    </span>
                  </Button>
                </Badge>
                {index < currentSelectedActions.length - 1 && (
                  <span className="material-symbols-outlined text-xs text-gray-400">
                    arrow_forward
                  </span>
                )}
              </div>
            ))}
          </div>
          
          <Separator />
        </div>
      )}

      {/* Actions列表 */}
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <h4 className="text-sm font-medium">
            可用 Actions ({filteredActions.length})
          </h4>
          {multiSelect && (
            <Badge variant="outline" className="text-xs">
              多选模式
            </Badge>
          )}
        </div>
        
        <div 
          className="action-list overflow-y-auto" 
          style={{ maxHeight: maxHeight || "300px" }}
        >
          {filteredActions.length > 0 ? (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {filteredActions.map(action => {
                const isSelected = currentSelectedActions.some(selected => selected.id === action.id);
                return (
                  <ActionCard
                    key={action.id}
                    action={action}
                    isSelected={isSelected}
                    selectionOrder={getSelectionOrder(action)}
                    onSelect={handleActionSelect}
                    onRemove={multiSelect ? handleActionRemove : undefined}
                    showSelection={true}
                  />
                );
              })}
            </div>
          ) : (
            <div className="text-center py-8 text-gray-500">
              <span className="material-symbols-outlined text-4xl mb-2 block">
                search_off
              </span>
              <p className="text-sm">未找到匹配的 Actions</p>
              <p className="text-xs text-gray-400 mt-1">尝试调整搜索条件或类型过滤器</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}