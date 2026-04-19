import { AlertTriangle, Info, Wifi, WifiOff } from "lucide-react";
import { useAppStore } from "../../stores/appStore";
import "./StatusBar.css";

export function StatusBar() {
  const { workspace, isOllamaRunning } = useAppStore();

  return (
    <div className="statusbar">
      <div className="statusbar-left">
        <div className="statusbar-item">
          <img src="/icons/vista/folder-blue.png" alt="" width={12} height={12} />
          <span>main</span>
        </div>
        <div className="statusbar-item">
          <AlertTriangle size={12} />
          <span>0</span>
          <Info size={12} />
          <span>0</span>
        </div>
      </div>

      <div className="statusbar-right">
        <div className="statusbar-item">
          {workspace ? workspace.name : "No project"}
        </div>
        <div className={`statusbar-item ${isOllamaRunning ? "connected" : "disconnected"}`}>
          {isOllamaRunning ? <Wifi size={12} /> : <WifiOff size={12} />}
          <span>{isOllamaRunning ? "AI Ready" : "AI Offline"}</span>
        </div>
        <div className="statusbar-item">
          <img src="/icons/msn/neutral.png" alt="" width={12} height={12} />
          <span>Ln 1, Col 1</span>
        </div>
        <div className="statusbar-item">
          <span>UTF-8</span>
        </div>
      </div>
    </div>
  );
}
