import { useAppStore } from "../../stores/appStore";
import { X } from "lucide-react";
import "./EditorTabs.css";

export function EditorTabs() {
  const { openTabs, activeTabId, setActiveTab, closeTab } = useAppStore();

  return (
    <div className="editor-tabs">
      {openTabs.map((tab) => (
        <div
          key={tab.id}
          className={`editor-tab ${tab.id === activeTabId ? "active" : ""}`}
          onClick={() => setActiveTab(tab.id)}
        >
          <span className="editor-tab-name">{tab.name}</span>
          {tab.is_dirty && <span className="editor-tab-dot" />}
          <button
            className="editor-tab-close"
            onClick={(e) => {
              e.stopPropagation();
              closeTab(tab.id);
            }}
          >
            <X size={12} />
          </button>
        </div>
      ))}
    </div>
  );
}
