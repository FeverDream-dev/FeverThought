import { useState } from "react";
import { FileText, ChevronDown, ChevronRight, Check, X, Bot } from "lucide-react";
import type { DiffEntry } from "../../stores/appStore";

interface DiffCardProps {
  diff: DiffEntry;
  onAccept: (id: string) => void;
  onReject: (id: string) => void;
}

export function DiffCard({ diff, onAccept, onReject }: DiffCardProps) {
  const [expanded, setExpanded] = useState(false);
  const statusClass = diff.status !== "pending" ? `diff-card--${diff.status}` : "";

  return (
    <div className={`diff-card ft-glass-panel ${statusClass}`}>
      <div className="diff-card-header">
        <div className="diff-card-file">
          <FileText size={14} className="diff-card-icon" />
          <span className="diff-card-filename">{diff.filePath.split("/").pop()}</span>
          <span className="diff-card-dir">{diff.filePath.split("/").slice(0, -1).join("/")}</span>
        </div>
        <div className="diff-card-actions">
          {diff.status === "pending" && (
            <>
              <button
                className="diff-card-btn diff-card-btn--accept"
                onClick={() => onAccept(diff.id)}
                title="Accept change"
              >
                <Check size={14} />
              </button>
              <button
                className="diff-card-btn diff-card-btn--reject"
                onClick={() => onReject(diff.id)}
                title="Reject change"
              >
                <X size={14} />
              </button>
            </>
          )}
          {diff.status !== "pending" && (
            <span className={`diff-card-status diff-card-status--${diff.status}`}>
              {diff.status === "accepted" ? "Accepted" : "Rejected"}
            </span>
          )}
        </div>
      </div>

      <div className="diff-card-summary">{diff.summary}</div>

      {diff.agentActionId && (
        <div className="diff-card-agent">
          <Bot size={12} />
          <span>Agent: {diff.agentActionId.slice(0, 8)}</span>
        </div>
      )}

      {diff.hunks.length > 0 && (
        <button
          className="diff-card-expand"
          onClick={() => setExpanded(!expanded)}
        >
          {expanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
          <span>{diff.hunks.length} hunk{diff.hunks.length !== 1 ? "s" : ""}</span>
        </button>
      )}

      {expanded && (
        <div className="diff-card-hunks">
          {diff.hunks.map((hunk: { oldStart: number; oldLines: number; newStart: number; newLines: number; content: string }, i: number) => (
            <div key={i} className="diff-hunk">
              <div className="diff-hunk-header">
                @@ -{hunk.oldStart},{hunk.oldLines} +{hunk.newStart},{hunk.newLines} @@
              </div>
              <pre className="diff-hunk-content">{hunk.content}</pre>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
