import { useState, useRef, useEffect } from "react";
import { useAppStore } from "../../stores/appStore";
import {
  Send,
  Sparkles,
  Wrench,
  ListChecks,
  GitCompare,
  HelpCircle,
  Brain,
  MessageCircle,
  Clock,
  CheckCircle,
  AlertCircle,
  Loader,
} from "lucide-react";
import { ClarificationWidget } from "../../ai/clarification/ClarificationWidget";
import { DiffCard } from "./DiffCard";
import type { AiPanelMode } from "../../stores/appStore";
import "./AiPanel.css";

const AI_MODES: { id: AiPanelMode; icon: typeof MessageCircle; label: string }[] = [
  { id: "chat", icon: MessageCircle, label: "Chat" },
  { id: "plan", icon: ListChecks, label: "Plan" },
  { id: "actions", icon: Wrench, label: "Actions" },
  { id: "diffs", icon: GitCompare, label: "Diffs" },
  { id: "clarification", icon: HelpCircle, label: "Clarify" },
  { id: "memory", icon: Brain, label: "Memory" },
];

export function AiPanel() {
  const {
    chatMessages,
    sendChatMessage,
    isOllamaRunning,
    checkOllama,
    aiPanelMode,
    setAiPanelMode,
    pendingDiffs,
    acceptDiff,
    rejectDiff,
  } = useAppStore();

  const [input, setInput] = useState("");
  const [showClarification, setShowClarification] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    checkOllama();
  }, [checkOllama]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [chatMessages]);

  const handleSend = async () => {
    if (!input.trim()) return;
    const msg = input;
    setInput("");
    await sendChatMessage(msg);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="ai-panel">
      <div className="ai-status-bar">
        <Sparkles size={14} className="ai-icon" />
        <span className="ai-status-text">AI Assistant</span>
        <span
          className={`ai-provider-badge ${
            isOllamaRunning ? "connected" : "disconnected"
          }`}
        >
          <span className="ai-provider-dot" />
          {isOllamaRunning ? "Ollama Connected" : "Ollama Offline"}
        </span>
      </div>

      <div className="ai-mode-tabs">
        {AI_MODES.map(({ id, icon: Icon, label }) => (
          <button
            key={id}
            className={`ai-mode-tab ${aiPanelMode === id ? "active" : ""}`}
            onClick={() => setAiPanelMode(id)}
            title={label}
          >
            <Icon size={13} />
            <span>{label}</span>
          </button>
        ))}
      </div>

      <div className="ai-panel-body">
        {aiPanelMode === "chat" && (
          <AiChatView
            messages={chatMessages}
            messagesEndRef={messagesEndRef}
            showClarification={showClarification}
            setShowClarification={setShowClarification}
          />
        )}
        {aiPanelMode === "plan" && <AiPlanView />}
        {aiPanelMode === "actions" && <AiActionsView />}
        {aiPanelMode === "diffs" && (
          <AiDiffsView
            diffs={pendingDiffs}
            onAccept={acceptDiff}
            onReject={rejectDiff}
          />
        )}
        {aiPanelMode === "clarification" && <AiClarifyView />}
        {aiPanelMode === "memory" && <AiMemoryView />}
      </div>

      {aiPanelMode === "chat" && (
        <div className="ai-input-area">
          <div className="ai-input-wrapper">
            <textarea
              className="ai-input"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Ask the AI assistant..."
              rows={1}
            />
            <button
              className="ai-send-button"
              onClick={handleSend}
              disabled={!input.trim()}
            >
              <Send size={16} />
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

// ── Chat View ──────────────────────────────────────────────

function AiChatView({
  messages,
  messagesEndRef,
  showClarification,
  setShowClarification,
}: {
  messages: ReturnType<typeof useAppStore.getState>["chatMessages"];
  messagesEndRef: React.RefObject<HTMLDivElement | null>;
  showClarification: boolean;
  setShowClarification: (v: boolean) => void;
}) {
  const [, setInput] = useState("");

  return (
    <>
      <div className="ai-messages">
        {messages.length === 0 && (
          <div className="ai-empty">
            <Sparkles size={32} className="ai-empty-icon" />
            <h3>Ask anything about your code</h3>
            <p>I can plan, refactor, debug, and build with you.</p>
            <div className="ai-suggestions">
              {["Explain this code", "Find bugs", "Add tests", "Refactor"].map(
                (s) => (
                  <button
                    key={s}
                    className="ft-button-secondary ai-suggestion-chip"
                    onClick={() => setInput(s)}
                  >
                    {s}
                  </button>
                )
              )}
            </div>
          </div>
        )}

        {messages.map((msg) => (
          <div key={msg.id} className={`ai-message ${msg.role}`}>
            <div className="ai-message-header">
              <div className="ai-message-avatar">
                {msg.role === "user" ? "U" : "✦"}
              </div>
              <div className="ai-message-meta">
                <span className="ai-message-role">
                  {msg.role === "user" ? "You" : "Assistant"}
                </span>
                {(msg.model || msg.provider) && (
                  <span className="ai-message-badge">
                    {msg.provider && (
                      <span className="ai-badge-provider">{msg.provider}</span>
                    )}
                    {msg.model && (
                      <span className="ai-badge-model">{msg.model}</span>
                    )}
                  </span>
                )}
                <span className="ai-message-time">
                  <Clock size={10} />
                  {new Date(msg.timestamp).toLocaleTimeString()}
                </span>
              </div>
            </div>

            <div className="ai-message-body">
              <div className="ai-message-content">{msg.content}</div>

              {msg.planSection && msg.planSection.length > 0 && (
                <div className="ai-message-plans">
                  {msg.planSection.map((plan, i) => (
                    <div key={i} className="ai-plan-section">
                      <span className="ai-plan-title">{plan.title}</span>
                      <ol className="ai-plan-steps">
                        {plan.steps.map((step, j) => (
                          <li key={j}>{step}</li>
                        ))}
                      </ol>
                    </div>
                  ))}
                </div>
              )}

              {msg.toolCalls && msg.toolCalls.length > 0 && (
                <div className="ai-message-tools">
                  {msg.toolCalls.map((tc, i) => (
                    <span
                      key={i}
                      className={`ai-tool-chip ai-tool-chip--${tc.status}`}
                    >
                      {tc.status === "running" && (
                        <Loader size={10} className="spinning" />
                      )}
                      {tc.status === "success" && <CheckCircle size={10} />}
                      {tc.status === "error" && <AlertCircle size={10} />}
                      {tc.status === "pending" && <Clock size={10} />}
                      {tc.toolName}
                      {tc.duration !== undefined && (
                        <span className="ai-tool-duration">
                          {tc.duration}ms
                        </span>
                      )}
                    </span>
                  ))}
                </div>
              )}

              {msg.diffRefs && msg.diffRefs.length > 0 && (
                <div className="ai-message-diff-refs">
                  {msg.diffRefs.map((ref, i) => (
                    <span key={i} className="ai-diff-ref">
                      <GitCompare size={10} />
                      {ref}
                    </span>
                  ))}
                </div>
              )}

              {msg.confidence !== undefined && (
                <div className="ai-message-confidence">
                  <div
                    className="ai-confidence-bar"
                    style={{
                      width: `${Math.round(msg.confidence * 100)}%`,
                    }}
                  />
                  <span>
                    Confidence: {Math.round(msg.confidence * 100)}%
                  </span>
                </div>
              )}

              {msg.assumptions && msg.assumptions.length > 0 && (
                <div className="ai-message-assumptions">
                  <span className="ai-assumptions-label">Assumptions:</span>
                  {msg.assumptions.map((a, i) => (
                    <span key={i} className="ai-assumption-tag">
                      {a}
                    </span>
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      {showClarification && (
        <ClarificationWidget
          question="What would you like me to help with?"
          options={[
            { label: "Build new feature", value: "feature" },
            { label: "Fix a bug", value: "bugfix" },
            { label: "Refactor code", value: "refactor" },
          ]}
          whyImAsking="I need to understand the scope before proposing changes."
          allowCustom={true}
          bestGuessAvailable={true}
          bestGuessValue="feature"
          onSelect={(value) => {
            setInput(value);
            setShowClarification(false);
          }}
          onDismiss={() => setShowClarification(false)}
        />
      )}
    </>
  );
}

// ── Plan View ──────────────────────────────────────────────

function AiPlanView() {
  const { agentSessions, activeAgentSessionId } = useAppStore();
  const activeSession = agentSessions.find(
    (s) => s.id === activeAgentSessionId
  );

  if (!activeSession?.plan) {
    return (
      <div className="ai-mode-empty">
        <ListChecks size={24} />
        <p>No active plan. Start a chat to generate one.</p>
      </div>
    );
  }

  return (
    <div className="ai-plan-view">
      <div className="ai-plan-summary">{activeSession.plan.summary}</div>
      <div className="ai-plan-meta">
        <span>Risk: {activeSession.plan.overallRisk}</span>
        <span>Files: {activeSession.plan.estimatedFiles}</span>
      </div>
      <div className="ai-plan-steps-list">
        {activeSession.plan.steps.map((step: { id: string; description: string; filePaths: string[]; riskTier: string; permission: unknown; status: string }) => (
          <div key={step.id} className={`ai-plan-step ai-plan-step--${step.status}`}>
            <span className="ai-plan-step-status">
              {step.status === "completed" && <CheckCircle size={12} />}
              {step.status === "in_progress" && <Loader size={12} className="spinning" />}
              {step.status === "failed" && <AlertCircle size={12} />}
              {step.status === "pending" && <Clock size={12} />}
              {step.status === "skipped" && <span>—</span>}
            </span>
            <span className="ai-plan-step-desc">{step.description}</span>
            <span className={`ai-plan-step-risk ai-plan-step-risk--${step.riskTier}`}>
              {step.riskTier}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

// ── Actions View ───────────────────────────────────────────

function AiActionsView() {
  const { agentSessions, activeAgentSessionId } = useAppStore();
  const activeSession = agentSessions.find(
    (s) => s.id === activeAgentSessionId
  );

  if (!activeSession || activeSession.actions.length === 0) {
    return (
      <div className="ai-mode-empty">
        <Wrench size={24} />
        <p>No actions recorded. Actions appear as the agent works.</p>
      </div>
    );
  }

  return (
    <div className="ai-actions-view">
      {activeSession.actions.map((action: { id: string; type: string; description: string; status: string; toolName?: string }) => (
        <div key={action.id} className={`ai-action-card ai-action--${action.status}`}>
          <div className="ai-action-header">
            <span className="ai-action-type">{action.type.replace(/_/g, " ")}</span>
            <span className={`ai-action-status ai-action-status--${action.status}`}>
              {action.status}
            </span>
          </div>
          <p className="ai-action-desc">{action.description}</p>
          {action.toolName && (
            <span className="ai-action-tool">
              <Wrench size={10} />
              {action.toolName}
            </span>
          )}
        </div>
      ))}
    </div>
  );
}

// ── Diffs View ─────────────────────────────────────────────

function AiDiffsView({
  diffs,
  onAccept,
  onReject,
}: {
  diffs: ReturnType<typeof useAppStore.getState>["pendingDiffs"];
  onAccept: (id: string) => void;
  onReject: (id: string) => void;
}) {
  if (diffs.length === 0) {
    return (
      <div className="ai-mode-empty">
        <GitCompare size={24} />
        <p>No pending diffs. Diffs appear when the agent proposes changes.</p>
      </div>
    );
  }

  return (
    <div className="ai-diffs-view">
      {diffs.map((diff) => (
        <DiffCard
          key={diff.id}
          diff={diff}
          onAccept={onAccept}
          onReject={onReject}
        />
      ))}
    </div>
  );
}

// ── Clarify View ───────────────────────────────────────────

function AiClarifyView() {
  return (
    <div className="ai-mode-empty">
      <HelpCircle size={24} />
      <p>Clarification prompts appear when the agent needs more info.</p>
    </div>
  );
}

// ── Memory View ────────────────────────────────────────────

function AiMemoryView() {
  const { agentSessions, activeAgentSessionId } = useAppStore();
  const activeSession = agentSessions.find(
    (s) => s.id === activeAgentSessionId
  );

  if (!activeSession) {
    return (
      <div className="ai-mode-empty">
        <Brain size={24} />
        <p>No active session memory. Start a chat to build context.</p>
      </div>
    );
  }

  return (
    <div className="ai-memory-view">
      <div className="ai-memory-section">
        <h4>Request</h4>
        <p>{activeSession.request}</p>
      </div>
      {activeSession.assumptions.length > 0 && (
        <div className="ai-memory-section">
          <h4>Assumptions</h4>
          <ul>
            {activeSession.assumptions.map((a: string, i: number) => (
              <li key={i}>{a}</li>
            ))}
          </ul>
        </div>
      )}
      <div className="ai-memory-section">
        <h4>Clarification</h4>
        <p>Status: {activeSession.clarificationState}</p>
      </div>
    </div>
  );
}
