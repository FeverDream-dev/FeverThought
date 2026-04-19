import { useState, useEffect, useRef, useCallback } from "react";
import { Search, FileText, Code, FolderOpen, Bot, Wrench, Palette, ChevronRight } from "lucide-react";
import "./CommandPalette.css";

interface CommandItem {
  id: string;
  label: string;
  category: string;
  icon: typeof FileText;
  shortcut?: string;
  action: () => void;
}

interface CommandPaletteProps {
  onClose: () => void;
  onOpenSettings: () => void;
  onToggleTheme: () => void;
  onNewFile: () => void;
  onOpenFile: () => void;
  onSwitchProvider: () => void;
}

const CATEGORY_ICONS: Record<string, typeof FileText> = {
  file: FileText,
  editor: Code,
  workspace: FolderOpen,
  ai: Bot,
  mcp: Wrench,
  provider: Bot,
  theme: Palette,
  navigation: ChevronRight,
};

export function CommandPalette({
  onClose,
  onOpenSettings,
  onToggleTheme,
  onNewFile,
  onOpenFile,
  onSwitchProvider,
}: CommandPaletteProps) {
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  const commands: CommandItem[] = [
    { id: "file.new", label: "New File", category: "file", icon: FileText, shortcut: "Ctrl+N", action: onNewFile },
    { id: "file.open", label: "Open File", category: "file", icon: FolderOpen, shortcut: "Ctrl+O", action: onOpenFile },
    { id: "file.save", label: "Save File", category: "file", icon: FileText, shortcut: "Ctrl+S", action: () => {} },
    { id: "file.save-all", label: "Save All", category: "file", icon: FileText, shortcut: "Ctrl+Shift+S", action: () => {} },
    { id: "editor.format", label: "Format Document", category: "editor", icon: Code, shortcut: "Shift+Alt+F", action: () => {} },
    { id: "editor.find", label: "Find in File", category: "editor", icon: Search, shortcut: "Ctrl+F", action: () => {} },
    { id: "editor.replace", label: "Find and Replace", category: "editor", icon: Search, shortcut: "Ctrl+H", action: () => {} },
    { id: "workspace.settings", label: "Open Settings", category: "workspace", icon: FileText, shortcut: "Ctrl+,", action: onOpenSettings },
    { id: "workspace.toggle-sidebar", label: "Toggle Sidebar", category: "workspace", icon: FolderOpen, shortcut: "Ctrl+B", action: () => {} },
    { id: "workspace.toggle-terminal", label: "Toggle Terminal", category: "workspace", icon: Code, shortcut: "Ctrl+`", action: () => {} },
    { id: "ai.chat", label: "New AI Chat", category: "ai", icon: Bot, action: () => {} },
    { id: "ai.plan", label: "Generate Plan", category: "ai", icon: Bot, action: () => {} },
    { id: "ai.review", label: "Review Changes", category: "ai", icon: Bot, action: () => {} },
    { id: "ai.explain", label: "Explain Code", category: "ai", icon: Bot, action: () => {} },
    { id: "mcp.open-tools", label: "Open MCP Tools", category: "mcp", icon: Wrench, action: () => {} },
    { id: "mcp.refresh", label: "Refresh MCP Servers", category: "mcp", icon: Wrench, action: () => {} },
    { id: "provider.switch", label: "Switch AI Provider", category: "provider", icon: Bot, action: onSwitchProvider },
    { id: "theme.toggle", label: "Toggle Theme", category: "theme", icon: Palette, action: onToggleTheme },
    { id: "nav.go-to-file", label: "Go to File", category: "navigation", icon: ChevronRight, shortcut: "Ctrl+P", action: () => {} },
    { id: "nav.go-to-line", label: "Go to Line", category: "navigation", icon: ChevronRight, shortcut: "Ctrl+G", action: () => {} },
    { id: "nav.go-to-symbol", label: "Go to Symbol", category: "navigation", icon: ChevronRight, shortcut: "Ctrl+Shift+O", action: () => {} },
  ];

  const filtered = query
    ? commands.filter(
        (c) =>
          c.label.toLowerCase().includes(query.toLowerCase()) ||
          c.category.toLowerCase().includes(query.toLowerCase())
      )
    : commands;

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  useEffect(() => {
    const item = listRef.current?.children[selectedIndex] as HTMLElement | undefined;
    item?.scrollIntoView({ block: "nearest" });
  }, [selectedIndex]);

  const executeSelected = useCallback(() => {
    if (filtered[selectedIndex]) {
      filtered[selectedIndex].action();
      onClose();
    }
  }, [filtered, selectedIndex, onClose]);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          setSelectedIndex((i) => Math.min(i + 1, filtered.length - 1));
          break;
        case "ArrowUp":
          e.preventDefault();
          setSelectedIndex((i) => Math.max(i - 1, 0));
          break;
        case "Enter":
          e.preventDefault();
          executeSelected();
          break;
        case "Escape":
          e.preventDefault();
          onClose();
          break;
      }
    },
    [filtered.length, executeSelected, onClose]
  );

  return (
    <div className="command-palette-overlay" onClick={onClose}>
      <div className="command-palette" onClick={(e) => e.stopPropagation()}>
        <div className="command-palette-input-wrapper">
          <Search size={16} className="command-palette-search-icon" />
          <input
            ref={inputRef}
            type="text"
            className="command-palette-input"
            placeholder="Type a command..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
          />
        </div>

        <div className="command-palette-list" ref={listRef}>
          {filtered.length === 0 && (
            <div className="command-palette-empty">No matching commands</div>
          )}
          {filtered.map((cmd, i) => {
            const Icon = CATEGORY_ICONS[cmd.category] ?? FileText;
            return (
              <button
                key={cmd.id}
                className={`command-palette-item ${i === selectedIndex ? "selected" : ""}`}
                onClick={() => {
                  cmd.action();
                  onClose();
                }}
                onMouseEnter={() => setSelectedIndex(i)}
              >
                <Icon size={14} className="command-palette-item-icon" />
                <span className="command-palette-item-label">{cmd.label}</span>
                <span className="command-palette-item-category">{cmd.category}</span>
                {cmd.shortcut && (
                  <kbd className="command-palette-shortcut">{cmd.shortcut}</kbd>
                )}
              </button>
            );
          })}
        </div>

        <div className="command-palette-footer">
          <span>↑↓ Navigate</span>
          <span>↵ Select</span>
          <span>Esc Close</span>
        </div>
      </div>
    </div>
  );
}
