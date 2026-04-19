// FeverThoth Shared Types
// Mirrors the Rust data models from the plan specification

// ── Workspace ──────────────────────────────────────────────

export interface Workspace {
  id: string;
  rootPath: string;
  name: string;
  openedAt: string;
  favorite: boolean;
  recentFiles: string[];
  detectedLanguages: string[];
}

// ── Provider ───────────────────────────────────────────────

export type ProviderKind =
  | "ollama"
  | "zai"
  | "ollama-cloud"
  | "gemini"
  | "codex"
  | "openrouter";

export interface ProviderProfile {
  id: string;
  provider: ProviderKind;
  enabled: boolean;
  defaultModel?: string;
  fallbackModel?: string;
  capabilityFlags: string[];
}

export interface ModelCapabilities {
  id: string;
  name: string;
  provider: ProviderKind;
  supportsVision: boolean;
  supportsToolCalling: boolean;
  supportsStreaming: boolean;
  contextWindow: number;
  maxOutputTokens: number;
}

// ── Agent Session ──────────────────────────────────────────

export interface AgentSession {
  id: string;
  workspaceId?: string;
  createdAt: string;
  request: string;
  assumptions: string[];
  clarificationState: ClarificationState;
  plan?: Plan;
  actions: AgentAction[];
}

export type ClarificationState = "none" | "pending" | "resolved";

// ── Plan ───────────────────────────────────────────────────

export interface Plan {
  id: string;
  steps: PlanStep[];
  overallRisk: RiskTier;
  estimatedFiles: number;
  summary: string;
}

export interface PlanStep {
  id: string;
  description: string;
  filePaths: string[];
  riskTier: RiskTier;
  permission: Permission;
  status: PlanStepStatus;
}

export type PlanStepStatus = "pending" | "in_progress" | "completed" | "failed" | "skipped";

// ── Risk & Permissions ─────────────────────────────────────

export type RiskTier = "low" | "medium" | "high";

export enum Permission {
  ReadWorkspaceFiles = "read_workspace_files",
  WriteWorkspaceFiles = "write_workspace_files",
  RunShellCommand = "run_shell_command",
  RunBrowserMcpTool = "run_browser_mcp_tool",
  SendDataToCloudProvider = "send_data_to_cloud_provider",
  UploadRawScreenshot = "upload_raw_screenshot",
  CommitChanges = "commit_changes",
  PushChanges = "push_changes",
}

// ── Agent Actions ──────────────────────────────────────────

export interface AgentAction {
  id: string;
  type: AgentActionType;
  description: string;
  status: AgentActionStatus;
  toolName?: string;
  toolInput?: Record<string, unknown>;
  toolOutput?: string;
  startedAt?: string;
  completedAt?: string;
}

export type AgentActionType =
  | "file_edit"
  | "file_create"
  | "shell_command"
  | "tool_call"
  | "screenshot_analysis";

export type AgentActionStatus =
  | "pending"
  | "running"
  | "completed"
  | "failed"
  | "cancelled";

// ── Chat Messages ──────────────────────────────────────────

export interface ChatMessage {
  id: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: number;
  /** Which model handled this message */
  model?: string;
  /** Which provider serviced this message */
  provider?: ProviderKind;
  /** Tool calls made during this message */
  toolCalls?: ToolCallChip[];
  /** Diff references produced by this message */
  diffRefs?: string[];
  /** Confidence level (0-1) the agent expressed */
  confidence?: number;
  /** Assumptions the agent made */
  assumptions?: string[];
  /** Plan sections embedded in the message */
  planSection?: PlanSection[];
}

export interface ToolCallChip {
  toolName: string;
  status: "pending" | "running" | "success" | "error";
  duration?: number;
}

export interface PlanSection {
  title: string;
  steps: string[];
}

// ── Clarification ──────────────────────────────────────────

export interface ClarificationQuestion {
  id: string;
  question: string;
  /** Explains to the user why the agent is asking */
  whyImAsking: string;
  options: ClarificationOption[];
  allowCustom: boolean;
  /** Whether a best-guess continue action is available */
  bestGuessAvailable: boolean;
  bestGuessValue?: string;
}

export interface ClarificationOption {
  label: string;
  value: string;
  description?: string;
}

// ── Screenshot Analysis ────────────────────────────────────

export interface ScreenshotAnalysis {
  id: string;
  source: "uploaded" | "captured";
  storedLocally: boolean;
  provider: "ollama";
  model: string;
  visibleText: string[];
  uiElements: string[];
  detectedErrors: string[];
  likelyContext: string;
  userGoalHypothesis: string[];
  ambiguityFlags: string[];
  followupQuestions: string[];
}

// ── Diff ───────────────────────────────────────────────────

export interface DiffEntry {
  id: string;
  filePath: string;
  summary: string;
  hunks: DiffHunk[];
  status: "pending" | "accepted" | "rejected";
  agentActionId?: string;
}

export interface DiffHunk {
  oldStart: number;
  oldLines: number;
  newStart: number;
  newLines: number;
  content: string;
}

// ── MCP ────────────────────────────────────────────────────

export interface McpServerStatus {
  name: string;
  installed: boolean;
  running: boolean;
  permissionGranted: boolean;
  toolCount: number;
  lastError?: string;
  lastStartedAt?: string;
}

// ── AI Panel Mode ──────────────────────────────────────────

export type AiPanelMode =
  | "chat"
  | "plan"
  | "actions"
  | "diffs"
  | "clarification"
  | "memory";

// ── Settings Sections ──────────────────────────────────────

export type SettingsSection =
  | "general"
  | "appearance"
  | "editor"
  | "keybindings"
  | "ai_providers"
  | "local_models"
  | "mcp_tools"
  | "privacy_security"
  | "git"
  | "terminal"
  | "experimental";

// ── Command Palette ────────────────────────────────────────

export type CommandCategory =
  | "file"
  | "editor"
  | "workspace"
  | "ai"
  | "mcp"
  | "provider"
  | "theme"
  | "navigation";

export interface CommandEntry {
  id: string;
  label: string;
  category: CommandCategory;
  shortcut?: string;
  action: () => void;
}

// ── Right Panel Tab ────────────────────────────────────────

export type RightPanelTab = "ai" | "outline" | "inspector";
