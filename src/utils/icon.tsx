import {
  LayoutGrid, Folder, FolderOpen, Terminal, Link, Bell, List, FileText,
  Settings, X, Search, Plus, ArrowRight, SearchX, ArrowLeft,
  ChevronUp, ChevronDown, TriangleAlert, RefreshCw, Loader2,
  Pencil, CirclePlus, Brain, Check, Play, CornerDownRight,
  Repeat, Clock, type LucideIcon,
} from "lucide-react";

const iconMap: Record<string, LucideIcon> = {
  apps: LayoutGrid,
  folder: Folder,
  folder_open: FolderOpen,
  terminal: Terminal,
  link: Link,
  notifications: Bell,
  view_list: List,
  description: FileText,
  settings: Settings,
  close: X,
  search: Search,
  add: Plus,
  arrow_forward: ArrowRight,
  search_off: SearchX,
  arrow_back: ArrowLeft,
  keyboard_arrow_up: ChevronUp,
  keyboard_arrow_down: ChevronDown,
  expand_less: ChevronUp,
  expand_more: ChevronDown,
  arrow_drop_up: ChevronUp,
  arrow_drop_down: ChevronDown,
  warning: TriangleAlert,
  refresh: RefreshCw,
  progress_activity: Loader2,
  edit: Pencil,
  add_circle: CirclePlus,
  psychology_alt: Brain,
  check: Check,
  autoplay: Play,
  subdirectory_arrow_right: CornerDownRight,
  repeat: Repeat,
  schedule: Clock,
};

export function ActionIcon({ name, size = 16, className = "" }: { name: string; size?: number; className?: string }) {
  const Icon = iconMap[name];
  if (!Icon) return null;
  return <Icon size={size} className={className} />;
}

export { iconMap };
