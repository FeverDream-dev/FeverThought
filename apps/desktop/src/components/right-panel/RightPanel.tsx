import { Sparkles, List, Search } from "lucide-react";
import { useAppStore } from "../../stores/appStore";
import { AiPanel } from "../../panels/ai/AiPanel";
import type { RightPanelTab } from "../../stores/appStore";
import "./RightPanel.css";

const RIGHT_TABS: { id: RightPanelTab; icon: typeof Sparkles; label: string }[] = [
  { id: "ai", icon: Sparkles, label: "AI" },
  { id: "outline", icon: List, label: "Outline" },
  { id: "inspector", icon: Search, label: "Inspector" },
];

export function RightPanel() {
  const { rightPanelTab, setRightPanelTab, rightPanelVisible, toggleRightPanel } = useAppStore();

  if (!rightPanelVisible) {
    return (
      <div className="right-panel-collapsed">
        <button className="right-panel-expand" onClick={toggleRightPanel} title="Show right panel">
          <Sparkles size={16} />
        </button>
      </div>
    );
  }

  return (
    <div className="right-panel">
      <div className="right-panel-tabs">
        {RIGHT_TABS.map(({ id, icon: Icon, label }) => (
          <button
            key={id}
            className={`right-panel-tab ${rightPanelTab === id ? "active" : ""}`}
            onClick={() => setRightPanelTab(id)}
            title={label}
          >
            <Icon size={14} />
            <span>{label}</span>
          </button>
        ))}
        <button
          className="right-panel-close"
          onClick={toggleRightPanel}
          title="Hide right panel"
        >
          ×
        </button>
      </div>

      <div className="right-panel-content">
        {rightPanelTab === "ai" && <AiPanel />}
        {rightPanelTab === "outline" && <OutlinePanel />}
        {rightPanelTab === "inspector" && <InspectorPanel />}
      </div>
    </div>
  );
}

function OutlinePanel() {
  const { workspace } = useAppStore();

  return (
    <div className="outline-panel">
      <div className="outline-empty">
        <List size={24} className="outline-empty-icon" />
        <p>{workspace ? "No symbol outline available" : "Open a file to see its outline"}</p>
      </div>
    </div>
  );
}

function InspectorPanel() {
  const { activeTabId, openTabs } = useAppStore();
  const activeTab = openTabs.find((t) => t.id === activeTabId);

  return (
    <div className="inspector-panel">
      {activeTab ? (
        <div className="inspector-content">
          <div className="inspector-field">
            <span className="inspector-label">File</span>
            <span className="inspector-value">{activeTab.path}</span>
          </div>
          <div className="inspector-field">
            <span className="inspector-label">Language</span>
            <span className="inspector-value">{activeTab.language}</span>
          </div>
          <div className="inspector-field">
            <span className="inspector-label">Modified</span>
            <span className="inspector-value">{activeTab.is_dirty ? "Yes" : "No"}</span>
          </div>
          <div className="inspector-field">
            <span className="inspector-label">Size</span>
            <span className="inspector-value">{activeTab.content.length} chars</span>
          </div>
        </div>
      ) : (
        <div className="inspector-empty">
          <Search size={24} className="inspector-empty-icon" />
          <p>Select an element to inspect</p>
        </div>
      )}
    </div>
  );
}
