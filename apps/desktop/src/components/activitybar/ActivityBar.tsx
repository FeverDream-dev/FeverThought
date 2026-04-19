import {
  FolderOpen,
  Search,
  GitBranch,
  Sparkles,
  Settings,
  Bug,
  Terminal,
  LayoutGrid,
} from "lucide-react";
import "./ActivityBar.css";

interface ActivityBarProps {
  activePanel: string;
  onPanelChange: (panel: string) => void;
}

const ACTIVITY_ITEMS = [
  { id: "explorer", icon: FolderOpen, label: "Explorer" },
  { id: "search", icon: Search, label: "Search" },
  { id: "git", icon: GitBranch, label: "Source Control" },
  { id: "ai", icon: Sparkles, label: "AI Assistant" },
  { id: "debug", icon: Bug, label: "Debug" },
  { id: "terminal", icon: Terminal, label: "Terminal" },
];

export function ActivityBar({ activePanel, onPanelChange }: ActivityBarProps) {
  return (
    <div className="activity-bar">
      <div className="activity-bar-top">
        {ACTIVITY_ITEMS.map(({ id, icon: Icon, label }) => (
          <button
            key={id}
            className={`activity-bar-item ${activePanel === id ? "active" : ""}`}
            onClick={() => onPanelChange(activePanel === id ? "" : id)}
            title={label}
          >
            <Icon size={22} strokeWidth={1.5} />
          </button>
        ))}
      </div>
      <div className="activity-bar-bottom">
        <button
          className="activity-bar-item"
          title="Settings"
        >
          <Settings size={22} strokeWidth={1.5} />
        </button>
        <button
          className="activity-bar-item"
          title="Layout"
        >
          <LayoutGrid size={22} strokeWidth={1.5} />
        </button>
      </div>
    </div>
  );
}
