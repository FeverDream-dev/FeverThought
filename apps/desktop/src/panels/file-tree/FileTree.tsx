import { useState, useCallback } from "react";
import { useAppStore } from "../../stores/appStore";
import { ChevronRight, ChevronDown, File, Folder, FolderOpen } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import "./FileTree.css";

interface FileTreeEntry {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileTreeEntry[];
}

export function FileTree() {
  const { workspace, openFile } = useAppStore();
  const [tree, setTree] = useState<FileTreeEntry[]>([]);
  const [expanded, setExpanded] = useState<Set<string>>(new Set());

  const loadRoot = useCallback(async () => {
    if (!workspace) return;
    try {
      const entries = await invoke<FileTreeEntry[]>("workspace_read_dir", {
        path: workspace.root_path,
      });
      setTree(entries);
    } catch (e) {
      console.error("Failed to load file tree:", e);
    }
  }, [workspace]);

  const toggleDir = useCallback(async (path: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }
      return next;
    });
  }, []);

  const renderEntry = (entry: FileTreeEntry, depth: number = 0) => {
    if (entry.is_dir) {
      const isExpanded = expanded.has(entry.path);
      return (
        <div key={entry.path}>
          <div
            className="file-tree-entry directory"
            style={{ paddingLeft: `${depth * 16 + 8}px` }}
            onClick={() => toggleDir(entry.path)}
          >
            {isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
            {isExpanded ? (
              <FolderOpen size={14} className="ft-icon-folder" />
            ) : (
              <Folder size={14} className="ft-icon-folder" />
            )}
            <span className="file-tree-name">{entry.name}</span>
          </div>
        </div>
      );
    }

    return (
      <div
        key={entry.path}
        className="file-tree-entry file"
        style={{ paddingLeft: `${depth * 16 + 24}px` }}
        onClick={() => openFile(entry.path)}
      >
        <File size={14} className="ft-icon-file" />
        <span className="file-tree-name">{entry.name}</span>
      </div>
    );
  };

  if (!workspace) {
    return (
      <div className="file-tree-empty">
        <p>No folder opened</p>
        <button className="ft-button-primary" onClick={loadRoot}>
          Open Folder
        </button>
      </div>
    );
  }

  if (tree.length === 0) {
    loadRoot();
  }

  return (
    <div className="file-tree">
      <div className="file-tree-section">
        <div className="file-tree-section-header">
          <span>{workspace.name.toUpperCase()}</span>
        </div>
      </div>
      {tree.map((entry) => renderEntry(entry))}
    </div>
  );
}
