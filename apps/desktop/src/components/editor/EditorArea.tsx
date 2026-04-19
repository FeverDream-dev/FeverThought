import { useAppStore } from "../../stores/appStore";
import { EditorTabs } from "../../editor/tabs/EditorTabs";
import { MonacoWrapper } from "../../editor/monaco/MonacoWrapper";
import { WelcomeScreen } from "../../editor/WelcomeScreen";
import "./EditorArea.css";

export function EditorArea() {
  const { openTabs, activeTabId, updateTabContent } = useAppStore();
  const activeTab = openTabs.find((t) => t.id === activeTabId);

  if (openTabs.length === 0) {
    return (
      <div className="editor-area">
        <WelcomeScreen />
      </div>
    );
  }

  return (
    <div className="editor-area">
      <EditorTabs />
      <div className="editor-content">
        {activeTab && (
          <MonacoWrapper
            key={activeTab.id}
            path={activeTab.path}
            language={activeTab.language}
            value={activeTab.content}
            onChange={(value) => updateTabContent(activeTab.id, value || "")}
          />
        )}
      </div>
    </div>
  );
}
