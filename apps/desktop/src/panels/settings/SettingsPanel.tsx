import { useState } from "react";
import {
  Settings,
  Palette,
  Code,
  Keyboard,
  Bot,
  Cpu,
  Wrench,
  ShieldCheck,
  GitBranch,
  Terminal,
  FlaskConical,
  ChevronRight,
  X,
} from "lucide-react";
import type { SettingsSection } from "../../stores/appStore";

interface SettingsPanelProps {
  onClose: () => void;
  initialSection?: SettingsSection;
}

const SECTIONS: { id: SettingsSection; label: string; icon: typeof Settings }[] = [
  { id: "general", label: "General", icon: Settings },
  { id: "appearance", label: "Appearance", icon: Palette },
  { id: "editor", label: "Editor", icon: Code },
  { id: "keybindings", label: "Keybindings", icon: Keyboard },
  { id: "ai_providers", label: "AI Providers", icon: Bot },
  { id: "local_models", label: "Local Models", icon: Cpu },
  { id: "mcp_tools", label: "MCP Tools", icon: Wrench },
  { id: "privacy_security", label: "Privacy & Security", icon: ShieldCheck },
  { id: "git", label: "Git", icon: GitBranch },
  { id: "terminal", label: "Terminal", icon: Terminal },
  { id: "experimental", label: "Experimental", icon: FlaskConical },
];

export function SettingsPanel({ onClose, initialSection = "general" }: SettingsPanelProps) {
  const [activeSection, setActiveSection] = useState<SettingsSection>(initialSection);
  const [searchQuery, setSearchQuery] = useState("");

  const filteredSections = searchQuery
    ? SECTIONS.filter((s) =>
        s.label.toLowerCase().includes(searchQuery.toLowerCase())
      )
    : SECTIONS;

  const ActiveIcon = SECTIONS.find((s) => s.id === activeSection)?.icon ?? Settings;

  return (
    <div className="settings-overlay">
      <div className="settings-panel">
        <div className="settings-sidebar">
          <div className="settings-search">
            <input
              type="text"
              placeholder="Search settings..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="settings-search-input"
            />
          </div>
          <nav className="settings-nav">
            {filteredSections.map(({ id, label, icon: Icon }) => (
              <button
                key={id}
                className={`settings-nav-item ${activeSection === id ? "active" : ""}`}
                onClick={() => setActiveSection(id)}
              >
                <Icon size={16} />
                <span>{label}</span>
                <ChevronRight size={14} className="settings-nav-arrow" />
              </button>
            ))}
          </nav>
        </div>

        <div className="settings-main">
          <div className="settings-header">
            <div className="settings-title">
              <ActiveIcon size={18} />
              <h2>{SECTIONS.find((s) => s.id === activeSection)?.label}</h2>
            </div>
            <button className="settings-close" onClick={onClose}>
              <X size={18} />
            </button>
          </div>

          <div className="settings-body">
            <SettingsSectionContent section={activeSection} />
          </div>
        </div>
      </div>
    </div>
  );
}

function SettingsSectionContent({ section }: { section: SettingsSection }) {
  switch (section) {
    case "general":
      return <GeneralSettings />;
    case "appearance":
      return <AppearanceSettings />;
    case "editor":
      return <EditorSettings />;
    case "keybindings":
      return <KeybindingsSettings />;
    case "ai_providers":
      return <AiProvidersSettings />;
    case "local_models":
      return <LocalModelsSettings />;
    case "mcp_tools":
      return <McpToolsSettings />;
    case "privacy_security":
      return <PrivacySecuritySettings />;
    case "git":
      return <GitSettings />;
    case "terminal":
      return <TerminalSettings />;
    case "experimental":
      return <ExperimentalSettings />;
    default:
      return null;
  }
}

function SettingRow({ label, description, children }: { label: string; description?: string; children: React.ReactNode }) {
  return (
    <div className="setting-row">
      <div className="setting-info">
        <span className="setting-label">{label}</span>
        {description && <span className="setting-desc">{description}</span>}
      </div>
      <div className="setting-control">{children}</div>
    </div>
  );
}

function GeneralSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Auto-save" description="Save files automatically on focus change">
        <select className="setting-select">
          <option>afterDelay</option>
          <option>onFocusChange</option>
          <option>off</option>
        </select>
      </SettingRow>
      <SettingRow label="Auto-save delay" description="Delay in milliseconds before auto-save">
        <input type="number" className="setting-input" defaultValue={1000} min={100} />
      </SettingRow>
      <SettingRow label="Startup behavior" description="What to show when FeverThoth starts">
        <select className="setting-select">
          <option>Welcome screen</option>
          <option>Empty editor</option>
          <option>Reopen last workspace</option>
        </select>
      </SettingRow>
      <SettingRow label="Update channel" description="Which release channel to receive updates from">
        <select className="setting-select">
          <option>Stable</option>
          <option>Beta</option>
          <option>Canary</option>
        </select>
      </SettingRow>
    </div>
  );
}

function AppearanceSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Theme" description="Choose between light and dark Frutiger Aero themes">
        <select className="setting-select">
          <option>Frutiger Light</option>
          <option>Frutiger Dark</option>
          <option>System</option>
        </select>
      </SettingRow>
      <SettingRow label="Font size" description="Base UI font size in pixels">
        <input type="number" className="setting-input" defaultValue={13} min={10} max={24} />
      </SettingRow>
      <SettingRow label="Zoom level" description="UI zoom level">
        <input type="number" className="setting-input" defaultValue={100} min={50} max={200} step={10} />
      </SettingRow>
      <SettingRow label="Glass effects" description="Enable translucent glass panel effects">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Animations" description="Enable UI animations and transitions">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
    </div>
  );
}

function EditorSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Tab size" description="Number of spaces per tab">
        <input type="number" className="setting-input" defaultValue={2} min={1} max={8} />
      </SettingRow>
      <SettingRow label="Word wrap" description="Enable word wrapping in the editor">
        <select className="setting-select">
          <option>off</option>
          <option>on</option>
          <option>wordWrapColumn</option>
        </select>
      </SettingRow>
      <SettingRow label="Minimap" description="Show the minimap sidebar in the editor">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Bracket pair colorization" description="Colorize matching bracket pairs">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Font family" description="Editor font family">
        <input type="text" className="setting-input" defaultValue="'Fira Code', 'Cascadia Code', monospace" />
      </SettingRow>
    </div>
  );
}

function KeybindingsSettings() {
  return (
    <div className="settings-section-content">
      <div className="settings-placeholder">
        <Keyboard size={32} />
        <p>Keybinding editor coming soon.</p>
        <p className="settings-placeholder-hint">Custom keybindings will be configurable here.</p>
      </div>
    </div>
  );
}

function AiProvidersSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Default provider" description="Primary AI provider for chat and actions">
        <select className="setting-select">
          <option>Ollama (Local)</option>
          <option>Z.AI Coding</option>
          <option>Google Gemini</option>
          <option>OpenRouter</option>
        </select>
      </SettingRow>
      <SettingRow label="Streaming" description="Stream AI responses token by token">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Agent auto-plan" description="Let the agent generate plans automatically before acting">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Clarification threshold" description="How often the agent should ask before acting">
        <select className="setting-select">
          <option>Always ask</option>
          <option>Ask when ambiguous</option>
          <option>Only ask for destructive actions</option>
        </select>
      </SettingRow>
    </div>
  );
}

function LocalModelsSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Default chat model" description="Model used for AI chat conversations">
        <select className="setting-select">
          <option>qwen2.5-coder:7b</option>
          <option>llama3.2:3b</option>
          <option>deepseek-coder:6.7b</option>
        </select>
      </SettingRow>
      <SettingRow label="Vision model" description="Model used for screenshot analysis">
        <select className="setting-select">
          <option>qwen2.5-vl:7b</option>
          <option>llama3.2-vision:11b</option>
        </select>
      </SettingRow>
      <SettingRow label="GPU layers" description="Number of layers to offload to GPU (-1 = auto)">
        <input type="number" className="setting-input" defaultValue={-1} min={-1} max={100} />
      </SettingRow>
    </div>
  );
}

function McpToolsSettings() {
  return (
    <div className="settings-section-content">
      <div className="settings-placeholder">
        <Wrench size={32} />
        <p>MCP tool configuration will appear here.</p>
        <p className="settings-placeholder-hint">Manage connected MCP servers, tools, and permissions.</p>
      </div>
    </div>
  );
}

function PrivacySecuritySettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Telemetry" description="Send anonymous usage data to improve FeverThoth">
        <label className="setting-toggle">
          <input type="checkbox" />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Screenshot cloud upload" description="Allow raw screenshots to be sent to cloud providers">
        <label className="setting-toggle">
          <input type="checkbox" />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Audit log" description="Log all agent actions for review">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Crash reports" description="Send crash reports automatically">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
    </div>
  );
}

function GitSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Auto-fetch" description="Fetch changes from remote periodically">
        <label className="setting-toggle">
          <input type="checkbox" defaultChecked />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Commit signing" description="Sign commits with GPG or SSH key">
        <label className="setting-toggle">
          <input type="checkbox" />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
    </div>
  );
}

function TerminalSettings() {
  return (
    <div className="settings-section-content">
      <SettingRow label="Shell" description="Default terminal shell">
        <select className="setting-select">
          <option>System default</option>
          <option>/bin/bash</option>
          <option>/bin/zsh</option>
          <option>/usr/bin/fish</option>
          <option>PowerShell</option>
        </select>
      </SettingRow>
      <SettingRow label="Font family" description="Terminal font family">
        <input type="text" className="setting-input" defaultValue="'Fira Code', monospace" />
      </SettingRow>
      <SettingRow label="Font size" description="Terminal font size">
        <input type="number" className="setting-input" defaultValue={14} min={8} max={32} />
      </SettingRow>
    </div>
  );
}

function ExperimentalSettings() {
  return (
    <div className="settings-section-content">
      <div className="settings-warning">
        <FlaskConical size={16} />
        <p>Experimental features may be unstable. Use at your own risk.</p>
      </div>
      <SettingRow label="GPU-accelerated terminal" description="Use WebGL renderer for terminal">
        <label className="setting-toggle">
          <input type="checkbox" />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
      <SettingRow label="Predictive completions" description="Show inline AI completions as you type">
        <label className="setting-toggle">
          <input type="checkbox" />
          <span className="setting-toggle-slider" />
        </label>
      </SettingRow>
    </div>
  );
}
