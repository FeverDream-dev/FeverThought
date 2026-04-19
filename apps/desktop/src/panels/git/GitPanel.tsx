import { GitBranch, GitCommit, RefreshCw } from "lucide-react";

export function GitPanel() {
  return (
    <div className="git-panel">
      <div className="git-branch-bar">
        <GitBranch size={14} />
        <span>main</span>
        <button className="ft-button-secondary" style={{ marginLeft: "auto", padding: "2px 8px", fontSize: "11px" }}>
          <RefreshCw size={12} />
        </button>
      </div>
      <div className="git-changes">
        <p className="ft-section-title">Changes</p>
        <p style={{ padding: "0 var(--ft-space-lg)", color: "var(--ft-text-tertiary)", fontSize: "12px" }}>
          No workspace open
        </p>
      </div>
    </div>
  );
}
