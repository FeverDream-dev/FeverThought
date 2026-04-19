import { open } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "../../stores/appStore";
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
          <svg width="80" height="80" viewBox="0 0 80 80" fill="none">
            <defs>
              <linearGradient id="welcome-grad" x1="0" y1="0" x2="80" y2="80">
                <stop stopColor="#4fc3f7" />
                <stop offset="0.5" stopColor="#2196f3" />
                <stop offset="1" stopColor="#1976d2" />
              </linearGradient>
              <filter id="welcome-glow">
                <feGaussianBlur stdDeviation="4" result="blur" />
                <feComposite in="SourceGraphic" in2="blur" operator="over" />
              </filter>
            </defs>
            <circle cx="40" cy="40" r="36" fill="url(#welcome-grad)" filter="url(#welcome-glow)" />
            <circle cx="40" cy="40" r="20" fill="white" opacity="0.15" />
            <circle cx="30" cy="30" r="8" fill="white" opacity="0.4" />
            <text x="40" y="48" textAnchor="middle" fill="white" fontSize="28" fontWeight="bold" fontFamily="sans-serif">
              FT
            </text>
          </svg>
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
