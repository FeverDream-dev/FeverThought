import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "../../stores/appStore";
import { Sparkles, CheckCircle, AlertCircle, FolderOpen, ArrowRight } from "lucide-react";
import "./OnboardingWizard.css";

interface OnboardingWizardProps {
  onComplete: () => void;
}

type Step = "welcome" | "ollama" | "project" | "done";

export function OnboardingWizard({ onComplete }: OnboardingWizardProps) {
  const [step, setStep] = useState<Step>("welcome");
  const [ollamaStatus, setOllamaStatus] = useState<"checking" | "connected" | "offline">("checking");
  const { openWorkspace, checkOllama } = useAppStore();

  const checkOllamaConnection = async () => {
    setOllamaStatus("checking");
    await checkOllama();
    const { isOllamaRunning } = useAppStore.getState();
    setOllamaStatus(isOllamaRunning ? "connected" : "offline");
  };

  const handleOpenProject = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select your project folder",
    });
    if (selected) {
      await openWorkspace(selected as string);
    }
  };

  return (
    <div className="onboarding">
      <div className="onboarding-bg" />
      <div className="onboarding-card ft-glass-panel">
        {step === "welcome" && (
          <div className="onboarding-step">
            <div className="onboarding-logo">
              <svg width="64" height="64" viewBox="0 0 80 80" fill="none">
                <defs>
                  <linearGradient id="ob-grad" x1="0" y1="0" x2="80" y2="80">
                    <stop stopColor="#4fc3f7" />
                    <stop offset="0.5" stopColor="#2196f3" />
                    <stop offset="1" stopColor="#1976d2" />
                  </linearGradient>
                </defs>
                <circle cx="40" cy="40" r="36" fill="url(#ob-grad)" />
                <text x="40" y="48" textAnchor="middle" fill="white" fontSize="28" fontWeight="bold">FT</text>
              </svg>
            </div>
            <h1>Welcome to FeverThoth IDE</h1>
            <p className="onboarding-subtitle">
              Your AI-first coding companion. Simple, fast, and thoughtful.
            </p>
            <div className="onboarding-features">
              <div className="feature-tag">
                <Sparkles size={14} /> AI-Powered
              </div>
              <div className="feature-tag">Rust Core</div>
              <div className="feature-tag">Cross-Platform</div>
              <div className="feature-tag">Privacy-First</div>
            </div>
            <button className="ft-button-primary onboarding-btn" onClick={() => setStep("ollama")}>
              Get Started <ArrowRight size={16} />
            </button>
          </div>
        )}

        {step === "ollama" && (
          <div className="onboarding-step">
            <h2>Connect AI</h2>
            <p className="onboarding-subtitle">
              FeverThoth uses Ollama for local AI. No data leaves your machine unless you choose to use cloud providers.
            </p>
            <div className="ollama-status-card">
              <button className="ft-button-primary" onClick={checkOllamaConnection}>
                Check Ollama Connection
              </button>
              {ollamaStatus === "checking" && <p className="status-text">Checking...</p>}
              {ollamaStatus === "connected" && (
                <div className="status-connected">
                  <CheckCircle size={20} /> Ollama is running!
                </div>
              )}
              {ollamaStatus === "offline" && (
                <div className="status-offline">
                  <AlertCircle size={20} />
                  <div>
                    <p>Ollama not detected.</p>
                    <p className="status-hint">Install from ollama.com and restart.</p>
                  </div>
                </div>
              )}
            </div>
            <div className="onboarding-nav">
              <button className="ft-button-secondary" onClick={() => setStep("welcome")}>Back</button>
              <button className="ft-button-primary" onClick={() => setStep("project")}>
                Continue <ArrowRight size={16} />
              </button>
            </div>
          </div>
        )}

        {step === "project" && (
          <div className="onboarding-step">
            <h2>Open a Project</h2>
            <p className="onboarding-subtitle">
              Select a folder to start coding, or skip to use the AI assistant first.
            </p>
            <div className="project-actions">
              <button className="ft-button-primary onboarding-btn" onClick={handleOpenProject}>
                <FolderOpen size={18} /> Open Folder
              </button>
            </div>
            <div className="onboarding-nav">
              <button className="ft-button-secondary" onClick={() => setStep("ollama")}>Back</button>
              <button className="ft-button-primary" onClick={() => setStep("done")}>
                Skip <ArrowRight size={16} />
              </button>
            </div>
          </div>
        )}

        {step === "done" && (
          <div className="onboarding-step">
            <h2>You're all set!</h2>
            <p className="onboarding-subtitle">
              Start coding with your AI assistant by your side.
            </p>
            <button className="ft-button-primary onboarding-btn" onClick={onComplete}>
              <Sparkles size={18} /> Launch FeverThoth
            </button>
          </div>
        )}

        <div className="onboarding-steps-indicator">
          {["welcome", "ollama", "project", "done"].map((s) => (
            <div
              key={s}
              className={`step-dot ${step === s ? "active" : ""}`}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
