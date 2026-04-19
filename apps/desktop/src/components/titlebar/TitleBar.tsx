import "./TitleBar.css";

export function TitleBar() {
  return (
    <div className="titlebar ft-glass-panel">
      <div className="titlebar-left">
        <div className="titlebar-logo">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="7" fill="url(#logo-grad)" />
            <circle cx="8" cy="8" r="4" fill="white" opacity="0.3" />
            <circle cx="6" cy="6" r="1.5" fill="white" opacity="0.6" />
            <defs>
              <linearGradient id="logo-grad" x1="0" y1="0" x2="16" y2="16">
                <stop stopColor="#4fc3f7" />
                <stop offset="1" stopColor="#1976d2" />
              </linearGradient>
            </defs>
          </svg>
        </div>
        <span className="titlebar-title">FeverThoth IDE</span>
      </div>
      <div className="titlebar-center" data-tauri-drag-region>
        <span className="titlebar-project" data-tauri-drag-region>
          No project open
        </span>
      </div>
      <div className="titlebar-right">
        <div className="titlebar-search">
          <input
            type="text"
            placeholder="Search files... (Ctrl+P)"
            className="titlebar-search-input"
          />
        </div>
      </div>
    </div>
  );
}
