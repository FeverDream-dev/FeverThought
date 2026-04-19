import {
  Search,
  GitBranch,
  Sparkles,
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
  { id: "explorer", iconSrc: "/icons/vista/folder-closed.png", label: "Explorer" },
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
        {ACTIVITY_ITEMS.map(({ id, icon: Icon, iconSrc, label }) => (
          <button
            key={id}
            className={`activity-bar-item ${activePanel === id ? "active" : ""}`}
            onClick={() => onPanelChange(activePanel === id ? "" : id)}
            title={label}
          >
            {iconSrc ? (
              <img src={iconSrc} alt={label} width={22} height={22} className="activity-bar-icon-img" />
            ) : Icon ? (
              <Icon size={22} strokeWidth={1.5} />
            ) : null}
          </button>
        ))}
      </div>
      <div className="activity-bar-bottom">
        <button
          className="activity-bar-item"
          title="Settings"
        >
          <img src="/icons/vista/settings.png" alt="Settings" width={20} height={20} className="activity-bar-icon-img" />
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
