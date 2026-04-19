import { open } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "../stores/appStore";
import { FolderOpen, Sparkles, Keyboard } from "lucide-react";

export function WelcomeScreen() {
  const { openWorkspace } = useAppStore();

  const handleOpenFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Open Project Folder",
    });
    if (selected) {
      await openWorkspace(selected as string);
    }
  };

  return (
    <div className="welcome-screen">
      <div className="welcome-content">
        <div className="welcome-logo">
          <img src="/icons/vista/app-icon.png" alt="FeverThoth" width={80} height={80} />
        </div>
        <h1 className="welcome-title">FeverThoth IDE</h1>
        <p className="welcome-subtitle">AI-first coding, reimagined.</p>

        <div className="welcome-actions">
          <button className="ft-button-primary welcome-btn" onClick={handleOpenFolder}>
            <FolderOpen size={18} />
            Open Project
          </button>
          <button className="ft-button-secondary welcome-btn">
            <Sparkles size={18} />
            Start with AI
          </button>
        </div>

        <div className="welcome-shortcuts">
          <h3>Quick Start</h3>
          <div className="shortcut-row">
            <Keyboard size={14} />
            <span>Ctrl+P</span>
            <span className="shortcut-desc">Quick open file</span>
          </div>
          <div className="shortcut-row">
            <Keyboard size={14} />
            <span>Ctrl+Shift+P</span>
            <span className="shortcut-desc">Command palette</span>
          </div>
          <div className="shortcut-row">
            <Keyboard size={14} />
            <span>Ctrl+`</span>
            <span className="shortcut-desc">Toggle terminal</span>
          </div>
        </div>
      </div>
    </div>
  );
}
