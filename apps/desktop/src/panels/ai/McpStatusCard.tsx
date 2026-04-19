import { useState } from "react";
import { Server, CircleDot, AlertTriangle, Wrench, FileText, ChevronRight } from "lucide-react";
import type { McpServerStatus } from "../../stores/appStore";

interface McpStatusCardProps {
  server: McpServerStatus;
  onOpenLogs: (serverName: string) => void;
}

export function McpStatusCard({ server, onOpenLogs }: McpStatusCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  const stateLabel = !server.installed
    ? "Not Installed"
    : !server.running
      ? "Stopped"
      : server.permissionGranted
        ? "Running"
        : "Awaiting Permission";

  const stateClass = !server.installed
    ? "mcp-state--unavailable"
    : !server.running
      ? "mcp-state--stopped"
      : server.permissionGranted
        ? "mcp-state--running"
        : "mcp-state--pending";

  return (
    <div className="mcp-status-card ft-glass-panel">
      <div className="mcp-card-header">
        <div className="mcp-card-name">
          <Server size={14} />
          <span>{server.name}</span>
        </div>
        <span className={`mcp-card-state ${stateClass}`}>
          <CircleDot size={10} />
          {stateLabel}
        </span>
      </div>

      <div className="mcp-card-meta">
        <div className="mcp-card-stat">
          <Wrench size={12} />
          <span>{server.toolCount} tool{server.toolCount !== 1 ? "s" : ""}</span>
        </div>
      </div>

      {server.lastError && (
        <div className="mcp-card-error">
          <AlertTriangle size={12} />
          <span>{server.lastError}</span>
        </div>
      )}

      <button className="mcp-card-expand" onClick={() => setShowDetails(!showDetails)}>
        <ChevronRight size={12} className={showDetails ? "rotated" : ""} />
        <span>Details</span>
      </button>

      {showDetails && (
        <div className="mcp-card-details">
          <div className="mcp-detail-row">
            <span>Installed</span>
            <span>{server.installed ? "Yes" : "No"}</span>
          </div>
          <div className="mcp-detail-row">
            <span>Running</span>
            <span>{server.running ? "Yes" : "No"}</span>
          </div>
          <div className="mcp-detail-row">
            <span>Permission</span>
            <span>{server.permissionGranted ? "Granted" : "Not granted"}</span>
          </div>
          {server.lastStartedAt && (
            <div className="mcp-detail-row">
              <span>Last started</span>
              <span>{new Date(server.lastStartedAt).toLocaleTimeString()}</span>
            </div>
          )}
        </div>
      )}

      <div className="mcp-card-footer">
        <button className="mcp-card-logs-btn" onClick={() => onOpenLogs(server.name)}>
          <FileText size={12} />
          <span>Open Logs</span>
        </button>
      </div>
    </div>
  );
}
