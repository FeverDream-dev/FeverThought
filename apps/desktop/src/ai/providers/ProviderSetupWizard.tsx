import { useState } from "react";
import { ArrowRight, CheckCircle, AlertCircle, Eye, EyeOff, Loader } from "lucide-react";
import type { ProviderKind } from "../../stores/appStore";

interface ProviderSetupWizardProps {
  onComplete: () => void;
  onDetectOllama: () => Promise<boolean>;
  onTestCloudProvider: (provider: ProviderKind, apiKey: string) => Promise<boolean>;
  onSaveProvider: (config: { provider: ProviderKind; apiKey?: string; model?: string }) => void;
}

type WizardStep = "detect" | "choose" | "configure-local" | "configure-cloud" | "done";

const CLOUD_PROVIDERS: { id: ProviderKind; name: string; description: string }[] = [
  { id: "zai", name: "Z.AI Coding", description: "Coding-specialized AI" },
  { id: "gemini", name: "Google Gemini", description: "Multimodal AI by Google" },
  { id: "openrouter", name: "OpenRouter", description: "Multi-model gateway" },
  { id: "codex", name: "Codex", description: "OpenAI Codex" },
];

export function ProviderSetupWizard({
  onComplete,
  onDetectOllama,
  onTestCloudProvider,
  onSaveProvider,
}: ProviderSetupWizardProps) {
  const [step, setStep] = useState<WizardStep>("detect");
  const [ollamaDetected, setOllamaDetected] = useState<boolean | null>(null);
  const [detecting, setDetecting] = useState(false);
  const [selectedProvider, setSelectedProvider] = useState<ProviderKind | null>(null);
  const [apiKey, setApiKey] = useState("");
  const [showKey, setShowKey] = useState(false);
  const [model, setModel] = useState("");
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<boolean | null>(null);

  const handleDetect = async () => {
    setDetecting(true);
    try {
      const found = await onDetectOllama();
      setOllamaDetected(found);
    } catch {
      setOllamaDetected(false);
    } finally {
      setDetecting(false);
    }
  };

  const handleTestCloud = async () => {
    if (!selectedProvider || !apiKey) return;
    setTesting(true);
    setTestResult(null);
    try {
      const ok = await onTestCloudProvider(selectedProvider, apiKey);
      setTestResult(ok);
    } catch {
      setTestResult(false);
    } finally {
      setTesting(false);
    }
  };

  const handleSave = () => {
    if (step === "configure-local") {
      onSaveProvider({ provider: "ollama", model: model || undefined });
    } else if (selectedProvider) {
      onSaveProvider({ provider: selectedProvider, apiKey, model: model || undefined });
    }
    setStep("done");
  };

  return (
    <div className="provider-wizard-overlay">
      <div className="provider-wizard ft-glass-panel">
        {step === "detect" && (
          <div className="provider-wizard-step">
            <h2>Detect Local AI</h2>
            <p className="provider-wizard-desc">
              FeverThoth uses Ollama for local, private AI. Let's check if it's installed.
            </p>
            <div className="provider-detect-area">
              <button className="ft-button-primary" onClick={handleDetect} disabled={detecting}>
                {detecting ? <Loader size={16} className="spinning" /> : null}
                {detecting ? "Scanning..." : "Detect Ollama"}
              </button>
              {ollamaDetected === true && (
                <div className="provider-detect-result success">
                  <CheckCircle size={20} /> Ollama detected and running!
                </div>
              )}
              {ollamaDetected === false && (
                <div className="provider-detect-result error">
                  <AlertCircle size={20} />
                  <div>
                    <p>Ollama not found.</p>
                    <p className="provider-hint">Install from ollama.com and try again, or set up a cloud provider.</p>
                  </div>
                </div>
              )}
            </div>
            <div className="provider-wizard-nav">
              {ollamaDetected && (
                <button
                  className="ft-button-primary"
                  onClick={() => setStep("configure-local")}
                >
                  Configure Ollama <ArrowRight size={16} />
                </button>
              )}
              <button className="ft-button-secondary" onClick={() => setStep("choose")}>
                Use Cloud Provider Instead <ArrowRight size={16} />
              </button>
            </div>
          </div>
        )}

        {step === "choose" && (
          <div className="provider-wizard-step">
            <h2>Choose a Cloud Provider</h2>
            <p className="provider-wizard-desc">Select an AI provider to connect.</p>
            <div className="provider-grid">
              {CLOUD_PROVIDERS.map((p) => (
                <button
                  key={p.id}
                  className={`provider-grid-card ${selectedProvider === p.id ? "selected" : ""}`}
                  onClick={() => setSelectedProvider(p.id)}
                >
                  <span className="provider-grid-name">{p.name}</span>
                  <span className="provider-grid-desc">{p.description}</span>
                </button>
              ))}
            </div>
            <div className="provider-wizard-nav">
              <button className="ft-button-secondary" onClick={() => setStep("detect")}>Back</button>
              <button
                className="ft-button-primary"
                disabled={!selectedProvider}
                onClick={() => setStep("configure-cloud")}
              >
                Continue <ArrowRight size={16} />
              </button>
            </div>
          </div>
        )}

        {step === "configure-local" && (
          <div className="provider-wizard-step">
            <h2>Configure Ollama</h2>
            <p className="provider-wizard-desc">Set your preferred model for local AI.</p>
            <div className="provider-form">
              <label className="provider-form-label">
                Default Model
                <input
                  type="text"
                  className="provider-form-input"
                  value={model}
                  onChange={(e) => setModel(e.target.value)}
                  placeholder="qwen2.5-coder:7b"
                />
              </label>
            </div>
            <div className="provider-wizard-nav">
              <button className="ft-button-secondary" onClick={() => setStep("detect")}>Back</button>
              <button className="ft-button-primary" onClick={handleSave}>
                Save & Continue <ArrowRight size={16} />
              </button>
            </div>
          </div>
        )}

        {step === "configure-cloud" && (
          <div className="provider-wizard-step">
            <h2>Configure {CLOUD_PROVIDERS.find((p) => p.id === selectedProvider)?.name}</h2>
            <p className="provider-wizard-desc">Enter your API key to connect.</p>
            <div className="provider-form">
              <label className="provider-form-label">
                API Key
                <div className="provider-form-input-row">
                  <input
                    type={showKey ? "text" : "password"}
                    className="provider-form-input"
                    value={apiKey}
                    onChange={(e) => {
                      setApiKey(e.target.value);
                      setTestResult(null);
                    }}
                    placeholder="Enter your API key..."
                  />
                  <button
                    className="provider-form-eye"
                    onClick={() => setShowKey(!showKey)}
                    type="button"
                  >
                    {showKey ? <EyeOff size={16} /> : <Eye size={16} />}
                  </button>
                </div>
              </label>
              <label className="provider-form-label">
                Model (optional)
                <input
                  type="text"
                  className="provider-form-input"
                  value={model}
                  onChange={(e) => setModel(e.target.value)}
                  placeholder="Default model for this provider"
                />
              </label>
            </div>
            <div className="provider-test-area">
              <button
                className="ft-button-secondary"
                onClick={handleTestCloud}
                disabled={!apiKey || testing}
              >
                {testing ? <Loader size={14} className="spinning" /> : null}
                {testing ? "Testing..." : "Test Connection"}
              </button>
              {testResult === true && (
                <span className="provider-test-success"><CheckCircle size={14} /> Connected!</span>
              )}
              {testResult === false && (
                <span className="provider-test-error"><AlertCircle size={14} /> Connection failed</span>
              )}
            </div>
            <div className="provider-wizard-nav">
              <button className="ft-button-secondary" onClick={() => setStep("choose")}>Back</button>
              <button className="ft-button-primary" onClick={handleSave}>
                Save & Continue <ArrowRight size={16} />
              </button>
            </div>
          </div>
        )}

        {step === "done" && (
          <div className="provider-wizard-step">
            <h2>Provider configured!</h2>
            <p className="provider-wizard-desc">You're ready to start using AI in FeverThoth.</p>
            <button className="ft-button-primary" onClick={onComplete}>
              Continue to IDE <ArrowRight size={16} />
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
