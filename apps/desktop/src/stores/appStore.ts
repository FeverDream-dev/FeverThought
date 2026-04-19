import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

// ── Re-export types from shared-types for convenience ──────

export type {
  ProviderProfile,
  ProviderKind,
  AgentSession,
  Plan,
  PlanStep,
  RiskTier,
  ClarificationState,
  AiPanelMode,
  RightPanelTab,
  SettingsSection,
  ToolCallChip,
  PlanSection,
  DiffEntry,
  DiffHunk,
  McpServerStatus,
  ScreenshotAnalysis,
  CommandEntry,
  CommandCategory,
} from "../../../packages/shared-types/src/index";

export { Permission } from "../../../packages/shared-types/src/index";

import type {
  ProviderProfile,
  AgentSession,
  AiPanelMode,
  RightPanelTab,
  SettingsSection,
  DiffEntry,
  McpServerStatus,
} from "../../../packages/shared-types/src/index";

// ── File & Tab types ───────────────────────────────────────

export interface FileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileEntry[];
  size?: number;
}

export interface TabInfo {
  id: string;
  path: string;
  name: string;
  language: string;
  is_dirty: boolean;
  content: string;
}

// ── Workspace ──────────────────────────────────────────────

export interface WorkspaceInfo {
  id: string;
  name: string;
  root_path: string;
  open_files: string[];
  favorite: boolean;
  detected_languages: string[];
}

// ── Chat Message (enhanced with plan metadata) ─────────────

export interface ChatMessage {
  id: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: number;
  /** Which model handled this message */
  model?: string;
  /** Which provider serviced this message */
  provider?: string;
  /** Tool calls made during this message */
  toolCalls?: { toolName: string; status: "pending" | "running" | "success" | "error"; duration?: number }[];
  /** Diff references produced by this message */
  diffRefs?: string[];
  /** Confidence level (0–1) */
  confidence?: number;
  /** Assumptions the agent made */
  assumptions?: string[];
  /** Plan sections embedded in the message */
  planSection?: { title: string; steps: string[] }[];
}

// ── Full Application State ─────────────────────────────────

interface AppState {
  // Workspace
  workspace: WorkspaceInfo | null;
  fileTree: FileEntry[];
  openTabs: TabInfo[];
  activeTabId: string | null;

  // AI / Chat
  chatMessages: ChatMessage[];
  isOllamaRunning: boolean;
  isLoading: boolean;
  aiPanelMode: AiPanelMode;

  // Providers
  providerProfiles: ProviderProfile[];
  activeProviderId: string | null;

  // Agent sessions
  agentSessions: AgentSession[];
  activeAgentSessionId: string | null;

  // Diffs
  pendingDiffs: DiffEntry[];

  // MCP
  mcpServers: McpServerStatus[];

  // UI layout state
  sidebarPanel: string;
  bottomPanel: string;
  rightPanelTab: RightPanelTab;
  rightPanelVisible: boolean;
  settingsOpen: boolean;
  settingsSection: SettingsSection;
  commandPaletteOpen: boolean;
  splashVisible: boolean;

  // Actions: Workspace
  openWorkspace: (path: string) => Promise<void>;
  closeWorkspace: () => Promise<void>;
  refreshFileTree: () => Promise<void>;
  openFile: (path: string) => Promise<void>;
  closeTab: (tabId: string) => void;
  setActiveTab: (tabId: string) => void;
  updateTabContent: (tabId: string, content: string) => void;

  // Actions: AI
  sendChatMessage: (message: string) => Promise<void>;
  checkOllama: () => Promise<void>;
  setAiPanelMode: (mode: AiPanelMode) => void;

  // Actions: Layout
  setSidebarPanel: (panel: string) => void;
  setBottomPanel: (panel: string) => void;
  setRightPanelTab: (tab: RightPanelTab) => void;
  toggleRightPanel: () => void;
  setSettingsOpen: (open: boolean, section?: SettingsSection) => void;
  setCommandPaletteOpen: (open: boolean) => void;
  setSplashVisible: (visible: boolean) => void;

  // Actions: Providers
  addProviderProfile: (profile: ProviderProfile) => void;
  updateProviderProfile: (id: string, updates: Partial<ProviderProfile>) => void;
  removeProviderProfile: (id: string) => void;
  setActiveProvider: (id: string) => void;

  // Actions: Diffs
  acceptDiff: (diffId: string) => void;
  rejectDiff: (diffId: string) => void;
}

export const useAppStore = create<AppState>((set, get) => ({
  // ── Initial State ──────────────────────────────────────

  workspace: null,
  fileTree: [],
  openTabs: [],
  activeTabId: null,
  chatMessages: [],
  isOllamaRunning: false,
  isLoading: false,
  aiPanelMode: "chat",

  providerProfiles: [],
  activeProviderId: null,

  agentSessions: [],
  activeAgentSessionId: null,

  pendingDiffs: [],
  mcpServers: [],

  sidebarPanel: "explorer",
  bottomPanel: "terminal",
  rightPanelTab: "ai",
  rightPanelVisible: true,
  settingsOpen: false,
  settingsSection: "general",
  commandPaletteOpen: false,
  splashVisible: true,

  // ── Workspace Actions ──────────────────────────────────

  openWorkspace: async (path: string) => {
    set({ isLoading: true });
    try {
      const ws = await invoke<WorkspaceInfo>("workspace_open", { path });
      set({ workspace: ws });
      await get().refreshFileTree();
    } catch (e) {
      console.error("Failed to open workspace:", e);
    } finally {
      set({ isLoading: false });
    }
  },

  closeWorkspace: async () => {
    await invoke("workspace_close");
    set({ workspace: null, fileTree: [], openTabs: [], activeTabId: null });
  },

  refreshFileTree: async () => {
    const { workspace } = get();
    if (!workspace) return;
    try {
      const entries = await invoke<FileEntry[]>("workspace_read_dir", {
        path: workspace.root_path,
      });
      set({ fileTree: entries });
    } catch (e) {
      console.error("Failed to read directory:", e);
    }
  },

  openFile: async (path: string) => {
    const existing = get().openTabs.find((t) => t.path === path);
    if (existing) {
      set({ activeTabId: existing.id });
      return;
    }

    try {
      const content = await invoke<string>("workspace_read_file", { path });
      const name = path.split("/").pop() || path.split("\\").pop() || path;
      const ext = name.includes(".") ? name.split(".").pop()! : "";
      const languageMap: Record<string, string> = {
        rs: "rust", ts: "typescript", tsx: "typescript",
        js: "javascript", jsx: "javascript", py: "python",
        go: "go", c: "c", h: "c", cpp: "cpp", hpp: "cpp",
        html: "html", css: "css", json: "json", md: "markdown",
        toml: "toml", yaml: "yaml", yml: "yaml",
      };

      const tab: TabInfo = {
        id: crypto.randomUUID(),
        path,
        name,
        language: languageMap[ext] || "plaintext",
        is_dirty: false,
        content,
      };

      set((s) => ({
        openTabs: [...s.openTabs, tab],
        activeTabId: tab.id,
      }));
    } catch (e) {
      console.error("Failed to open file:", e);
    }
  },

  closeTab: (tabId: string) => {
    set((s) => {
      const tabs = s.openTabs.filter((t) => t.id !== tabId);
      const activeTabId =
        s.activeTabId === tabId
          ? tabs[tabs.length - 1]?.id || null
          : s.activeTabId;
      return { openTabs: tabs, activeTabId };
    });
  },

  setActiveTab: (tabId: string) => set({ activeTabId: tabId }),

  updateTabContent: (tabId: string, content: string) => {
    set((s) => ({
      openTabs: s.openTabs.map((t) =>
        t.id === tabId ? { ...t, content, is_dirty: true } : t
      ),
    }));
  },

  // ── AI Actions ─────────────────────────────────────────

  sendChatMessage: async (message: string) => {
    const userMsg: ChatMessage = {
      id: crypto.randomUUID(),
      role: "user",
      content: message,
      timestamp: Date.now(),
    };

    set((s) => ({ chatMessages: [...s.chatMessages, userMsg] }));

    try {
      const response = await invoke<string>("ai_chat", { message });
      const assistantMsg: ChatMessage = {
        id: crypto.randomUUID(),
        role: "assistant",
        content: response,
        timestamp: Date.now(),
      };
      set((s) => ({ chatMessages: [...s.chatMessages, assistantMsg] }));
    } catch (e) {
      const errorMsg: ChatMessage = {
        id: crypto.randomUUID(),
        role: "assistant",
        content: `Error: ${e}`,
        timestamp: Date.now(),
      };
      set((s) => ({ chatMessages: [...s.chatMessages, errorMsg] }));
    }
  },

  checkOllama: async () => {
    try {
      const running = await invoke<boolean>("ai_check_ollama");
      set({ isOllamaRunning: running });
    } catch {
      set({ isOllamaRunning: false });
    }
  },

  setAiPanelMode: (mode: AiPanelMode) => set({ aiPanelMode: mode }),

  // ── Layout Actions ─────────────────────────────────────

  setSidebarPanel: (panel: string) => set({ sidebarPanel: panel }),
  setBottomPanel: (panel: string) => set({ bottomPanel: panel }),
  setRightPanelTab: (tab: RightPanelTab) => set({ rightPanelTab: tab }),
  toggleRightPanel: () =>
    set((s) => ({ rightPanelVisible: !s.rightPanelVisible })),
  setSettingsOpen: (open: boolean, section?: SettingsSection) =>
    set({ settingsOpen: open, settingsSection: section ?? "general" }),
  setCommandPaletteOpen: (open: boolean) => set({ commandPaletteOpen: open }),
  setSplashVisible: (visible: boolean) => set({ splashVisible: visible }),

  // ── Provider Actions ───────────────────────────────────

  addProviderProfile: (profile: ProviderProfile) =>
    set((s) => ({ providerProfiles: [...s.providerProfiles, profile] })),

  updateProviderProfile: (id: string, updates: Partial<ProviderProfile>) =>
    set((s) => ({
      providerProfiles: s.providerProfiles.map((p) =>
        p.id === id ? { ...p, ...updates } : p
      ),
    })),

  removeProviderProfile: (id: string) =>
    set((s) => ({
      providerProfiles: s.providerProfiles.filter((p) => p.id !== id),
      activeProviderId: s.activeProviderId === id ? null : s.activeProviderId,
    })),

  setActiveProvider: (id: string) => set({ activeProviderId: id }),

  // ── Diff Actions ───────────────────────────────────────

  acceptDiff: (diffId: string) =>
    set((s) => ({
      pendingDiffs: s.pendingDiffs.map((d) =>
        d.id === diffId ? { ...d, status: "accepted" as const } : d
      ),
    })),

  rejectDiff: (diffId: string) =>
    set((s) => ({
      pendingDiffs: s.pendingDiffs.map((d) =>
        d.id === diffId ? { ...d, status: "rejected" as const } : d
      ),
    })),
}));
