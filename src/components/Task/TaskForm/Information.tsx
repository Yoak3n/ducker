import React from "react";
import { Checkbox } from "@/components/ui/checkbox";
import type { InputHandle } from "./type";
import type { Task } from "@/types";

interface Props {
    editing: boolean;
    completed: boolean;
    name: string;
    auto: boolean;  
    value?: number;
    parentTask?: Task;
    handleInputChange: InputHandle;
}

const Information = React.memo(function Information({ editing, completed, name, auto, value, parentTask, handleInputChange }: Props) {
    return (
        <div className="bg-gradient-to-br from-slate-50 to-slate-100 border border-slate-300 rounded-lg p-4 transition-all duration-200 hover:border-slate-400 hover:shadow-md">
          <div className="flex items-center gap-2 mb-3 pb-2 border-b border-gray-200 bg-white/80 rounded px-3 py-2 -mx-2">
            <span className="material-symbols-outlined text-slate-800 text-lg">edit</span>
            <h3 className="m-0 text-sm font-semibold text-gray-800">基本信息</h3>
          </div>

          {/* 任务状态、标题和自动执行在同一行 */}
          <div className={`grid gap-3 mb-4 items-end ${editing ? 'grid-cols-[auto_1fr_auto]' : 'grid-cols-[1fr_auto]'}`}>
            {/* 任务完成状态 - 仅在编辑模式时显示 */}
            {editing && (
              <div className="flex flex-col gap-2 min-w-[100px]">
                <label className="text-sm font-medium text-gray-600 m-0">任务状态</label>
                <div className='flex items-center gap-2 px-3 py-2.5 bg-white border border-gray-200 rounded-lg transition-all duration-200 cursor-pointer hover:bg-gray-50 hover:border-gray-300 hover:-translate-y-0.5 hover:shadow-lg h-10 justify-center'>
                  <Checkbox
                    checked={completed}
                    onCheckedChange={(v) => handleInputChange('completed', v)}
                  />
                  <label className="text-sm cursor-pointer select-none">已完成</label>
                </div>
              </div>
            )}

            {/* 任务标题 */}
            <div className="flex-1 min-w-0">
              <label htmlFor="title" className="block mb-2 font-medium text-gray-600 text-sm">任务标题 *</label>
              <input
                type="text"
                id="title"
                value={name}
                onChange={(e) => handleInputChange('name', e.target.value)}
                placeholder="输入任务标题..."
                autoComplete='off'
                required
                autoFocus
                className="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm transition-all duration-200 bg-white focus:outline-none focus:border-blue-500 focus:shadow-[0_0_0_3px_rgba(59,130,246,0.1)] h-10 placeholder:text-gray-400"
              />
            </div>

            {/* 自动执行 */}
            <div className="flex flex-col gap-2 min-w-[100px]">
              <label className="text-sm font-medium text-gray-600 m-0">自动执行</label>
              <div className='flex items-center gap-2 px-3 py-2.5 bg-slate-50 border border-slate-200 rounded-lg transition-all duration-200 cursor-pointer h-10 justify-center hover:bg-slate-100 hover:border-slate-300 hover:-translate-y-0.5 hover:shadow-lg'>
                <Checkbox
                  checked={auto}
                  onCheckedChange={(v) => handleInputChange('auto', v)}
                />
                <span className="text-sm font-medium text-slate-600 m-0 cursor-pointer select-none">启用</span>
              </div>
            </div>
          </div>

          {parentTask && (
            <div className="flex items-center gap-2 px-3 py-2 bg-sky-50 border border-sky-200 rounded-md mb-3 text-sky-700 text-sm">
              <span className="material-symbols-outlined">subdirectory_arrow_right</span>
              <span>父任务: {parentTask.name}</span>
            </div>
          )}

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div className="mb-3">
              <label htmlFor="task-value" className="block mb-2 font-medium text-gray-600 text-sm">任务价值</label>
              <input
                id="task-value"
                type="number"
                value={value || ''}
                onChange={(e) => handleInputChange('value', e.target.value ? Number(e.target.value) : undefined)}
                placeholder="请输入任务价值（可选）"
                min="0"
                step="1"
                className="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm transition-all duration-200 bg-white focus:outline-none focus:border-blue-500 focus:shadow-[0_0_0_3px_rgba(59,130,246,0.1)] h-10 placeholder:text-gray-400"
              />
            </div>
            <div className="mb-3">
              <label htmlFor="parent-task" className="block mb-2 font-medium text-gray-600 text-sm">父任务ID</label>
              <input
                id="parent-task"
                type="text"
                value={parentTask?.id || ''}
                onChange={(e) => handleInputChange('parent_id', e.target.value || undefined)}
                placeholder="请输入父任务ID（可选）"
                className="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm transition-all duration-200 bg-white focus:outline-none focus:border-blue-500 focus:shadow-[0_0_0_3px_rgba(59,130,246,0.1)] h-10 placeholder:text-gray-400"
              />
            </div>
          </div>
        </div>
    )
});

export default Information;