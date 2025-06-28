import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import type{ Action } from "@/types/modules/action"
import { actionTypes } from "@/types"

interface ActionCardProps {
  action: Action;
  isSelected?: boolean;
  selectionOrder?: number;
  onSelect?: (action: Action) => void;
  onRemove?: (action: Action) => void;
  selectable?: boolean;
}

export default function ActionCard({ 
  action, 
  isSelected = false, 
  selectionOrder, 
  onSelect, 
  onRemove,
  selectable = false 
}: ActionCardProps) {
  const actionType = actionTypes.find(type => type.value === action.type);
  
  const handleCardClick = () => {
    if (selectable && onSelect) {
      onSelect(action);
    }
  };

  const handleRemove = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (onRemove) {
      onRemove(action);
    }
  };

  return (
    <Card 
      className={`box-border cursor-pointer transition-all duration-200 hover:shadow-md ${
        isSelected ? 'ring-2 ring-blue-500 bg-blue-50' : 'hover:bg-gray-50'
      }`}
      onClick={handleCardClick}
    >
      <CardHeader className="pb-2 h-6">
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-2">
            <span className="material-symbols-outlined text-lg">
              {actionType?.icon || 'settings'}
            </span>
            <CardTitle className="text-sm font-medium">{action.name}</CardTitle>
            {isSelected && selectionOrder !== undefined && (
              <Badge variant="secondary" className="text-xs p-0">
                {selectionOrder}
              </Badge>
            )}
          </div>
          {isSelected && onRemove && (
            <CardAction className="m-0">
              <Button 
                variant="ghost" 
                size="sm" 
                onClick={handleRemove}
                className="h-6 w-6 p-0 hover:bg-red-100"
              >
                <span className="material-symbols-outlined text-sm text-red-500">
                  close
                </span>
              </Button>
            </CardAction>
          )}
        </div>
      </CardHeader>
      
      <CardContent className="pt-0 pb-2">
        <CardDescription className="text-xs text-gray-600 line-clamp-2">
          {action.desc}
        </CardDescription>
        
        <div className="mt-2 flex items-center gap-2">
          <Badge variant="outline" className="text-xs">
            {actionType?.label || action.type}
          </Badge>
          {action.wait > 0 && (
            <Badge variant="secondary" className="text-xs">
              等待 {action.wait}ms
            </Badge>
          )}
        </div>
      </CardContent>
      
      <CardFooter className="pt-0 pb-3">
        <div className="w-full">
          <div className="text-xs text-gray-500 font-mono truncate">
            {action.command} {action.args?.join(' ')}
          </div>
        </div>
      </CardFooter>
    </Card>
  )
}