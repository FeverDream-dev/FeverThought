import { FileTree } from "../../panels/file-tree/FileTree";
import { SearchPanel } from "../../panels/search/SearchPanel";
import { GitPanel } from "../../panels/git/GitPanel";
import "./Sidebar.css";

interface SidebarProps {
  activePanel: string;
}

export function Sidebar({ activePanel }: SidebarProps) {
  if (!activePanel) return null;

  return (
    <div className="sidebar">
      <div className="sidebar-header">
        {activePanel === "explorer" && (
          <img src="/icons/wii/home.png" alt="" width={14} height={14} className="sidebar-header-icon" />
        )}
        <h2 className="sidebar-title">
          {activePanel === "explorer" && "Explorer"}
          {activePanel === "search" && "Search"}
          {activePanel === "git" && "Source Control"}
          {activePanel === "debug" && "Debug"}
          {activePanel === "terminal" && "Terminal"}
        </h2>
      </div>
      <div className="sidebar-content">
        {activePanel === "explorer" && <FileTree />}
        {activePanel === "search" && <SearchPanel />}
        {activePanel === "git" && <GitPanel />}
      </div>
    </div>
  );
}
