import "./TitleBar.css";

export function TitleBar() {
  return (
    <div className="titlebar ft-glass-panel">
      <div className="titlebar-left">
        <div className="titlebar-logo">
          <img src="/icons/vista/app-icon.png" alt="FeverThoth" width={16} height={16} />
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
