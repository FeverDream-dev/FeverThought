import { useState } from "react";
import { Wifi, WifiOff, RefreshCw, Star, Shield } from "lucide-react";
import type { ProviderProfile } from "../../stores/appStore";

interface ProviderCardProps {
  profile: ProviderProfile;
  isActive: boolean;
  onSetActive: (id: string) => void;
  onTestConnection: (id: string) => Promise<boolean>;
  onUpdate: (id: string, updates: Partial<ProviderProfile>) => void;
}

const PROVIDER_LABELS: Record<string, string> = {
  ollama: "Ollama",
  zai: "Z.AI Coding",
  "ollama-cloud": "Ollama Cloud",
  gemini: "Google Gemini",
  codex: "Codex",
  openrouter: "OpenRouter",
};

export function ProviderCard({
  profile,
  isActive,
  onSetActive,
  onTestConnection,
  onUpdate,
}: ProviderCardProps) {
  const [testing, setTesting] = useState(false);
  const [connected, setConnected] = useState<boolean | null>(null);

  const handleTest = async () => {
    setTesting(true);
    try {
      const result = await onTestConnection(profile.id);
      setConnected(result);
    } catch {
      setConnected(false);
    } finally {
      setTesting(false);
    }
  };

  return (
    <div className={`provider-card ft-glass-panel ${isActive ? "provider-card--active" : ""}`}>
      <div className="provider-card-header">
        <div className="provider-card-name">
          {connected === true ? (
            <Wifi size={14} className="provider-card-status provider-card-status--on" />
          ) : connected === false ? (
            <WifiOff size={14} className="provider-card-status provider-card-status--off" />
          ) : (
            <Shield size={14} className="provider-card-status" />
          )}
          <span>{PROVIDER_LABELS[profile.provider] ?? profile.provider}</span>
          {!profile.enabled && <span className="provider-card-disabled-badge">Disabled</span>}
        </div>
        <button
          className={`provider-card-set-default ${isActive ? "active" : ""}`}
          onClick={() => onSetActive(profile.id)}
          title={isActive ? "Active provider" : "Set as active"}
        >
          <Star size={14} fill={isActive ? "currentColor" : "none"} />
        </button>
      </div>

      <div className="provider-card-models">
        {profile.defaultModel && (
          <div className="provider-card-model">
            <span className="provider-card-model-label">Default</span>
            <span className="provider-card-model-value">{profile.defaultModel}</span>
          </div>
        )}
        {profile.fallbackModel && (
          <div className="provider-card-model">
            <span className="provider-card-model-label">Fallback</span>
            <span className="provider-card-model-value">{profile.fallbackModel}</span>
          </div>
        )}
      </div>

      {profile.capabilityFlags.length > 0 && (
        <div className="provider-card-capabilities">
          {profile.capabilityFlags.map((flag: string) => (
            <span key={flag} className="provider-card-cap">{flag}</span>
          ))}
        </div>
      )}

      <div className="provider-card-footer">
        <button
          className="ft-button-secondary provider-card-test"
          onClick={handleTest}
          disabled={testing || !profile.enabled}
        >
          <RefreshCw size={12} className={testing ? "spinning" : ""} />
          {testing ? "Testing..." : "Test Connection"}
        </button>

        <label className="provider-card-toggle">
          <input
            type="checkbox"
            checked={profile.enabled}
            onChange={(e) => onUpdate(profile.id, { enabled: e.target.checked })}
          />
          <span>Enabled</span>
        </label>
      </div>
    </div>
  );
}
