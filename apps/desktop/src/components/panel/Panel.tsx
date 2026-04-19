import { Terminal as TerminalPanel, Search, X } from "lucide-react";
import "./Panel.css";

interface PanelProps {
  activePanel: string;
  onPanelChange: (panel: string) => void;
}

export function Panel({ activePanel, onPanelChange }: PanelProps) {
  if (!activePanel) return null;

  return (
    <div className="bottom-panel">
      <div className="panel-tabs">
        {["terminal", "problems", "output"].map((id) => (
          <button
            key={id}
            className={`panel-tab ${activePanel === id ? "active" : ""}`}
            onClick={() => onPanelChange(id)}
          >
            {id.charAt(0).toUpperCase() + id.slice(1)}
          </button>
        ))}
        <button
          className="panel-close"
          onClick={() => onPanelChange("")}
        >
          <X size={14} />
        </button>
      </div>
      <div className="panel-content">
        {activePanel === "terminal" && (
          <div className="terminal-placeholder">
            <TerminalPanel size={16} />
            <span>Terminal — press Ctrl+` to toggle</span>
          </div>
        )}
        {activePanel === "problems" && (
          <div className="problems-placeholder">
            <span>No problems detected</span>
          </div>
        )}
        {activePanel === "output" && (
          <div className="output-placeholder">
            <span>No output</span>
          </div>
        )}
      </div>
    </div>
  );
}
