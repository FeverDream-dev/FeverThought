import { useState } from "react";
import { MessageCircle, Lightbulb } from "lucide-react";
import "./ClarificationWidget.css";

interface ClarificationOption {
  label: string;
  value: string;
  description?: string;
}

interface ClarificationWidgetProps {
  question: string;
  options: ClarificationOption[];
  allowCustom?: boolean;
  whyImAsking?: string;
  bestGuessAvailable?: boolean;
  bestGuessValue?: string;
  onSelect: (value: string, rememberForSession?: boolean) => void;
  onDismiss: () => void;
}

export function ClarificationWidget({
  question,
  options,
  allowCustom = true,
  whyImAsking,
  bestGuessAvailable = false,
  bestGuessValue,
  onSelect,
  onDismiss,
}: ClarificationWidgetProps) {
  const [customInput, setCustomInput] = useState("");
  const [selected, setSelected] = useState<string | null>(null);
  const [rememberForSession, setRememberForSession] = useState(false);

  const handleSelect = (value: string) => {
    setSelected(value);
    onSelect(value, rememberForSession);
  };

  const handleCustomSubmit = () => {
    if (customInput.trim()) {
      onSelect(customInput.trim(), rememberForSession);
      setCustomInput("");
    }
  };

  const handleBestGuess = () => {
    if (bestGuessValue) {
      setSelected(bestGuessValue);
      onSelect(bestGuessValue, rememberForSession);
    }
  };

  return (
    <div className="clarification-widget ft-glass-panel">
      <div className="clarification-header">
        <MessageCircle size={16} className="clarification-icon" />
        <div className="clarification-header-text">
          <span className="clarification-question">{question}</span>
          {whyImAsking && (
            <div className="clarification-why">
              <Lightbulb size={12} />
              <span>{whyImAsking}</span>
            </div>
          )}
        </div>
      </div>

      <div className="clarification-options">
        {options.map((option) => (
          <button
            key={option.value}
            className={`clarification-option ${
              selected === option.value ? "selected" : ""
            }`}
            onClick={() => handleSelect(option.value)}
          >
            <span className="clarification-option-label">{option.label}</span>
            {option.description && (
              <span className="clarification-option-desc">
                {option.description}
              </span>
            )}
          </button>
        ))}
      </div>

      {allowCustom && (
        <div className="clarification-custom">
          <input
            type="text"
            value={customInput}
            onChange={(e) => setCustomInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleCustomSubmit()}
            placeholder="Type your own answer..."
            className="clarification-input"
          />
          <button
            className="ft-button-primary clarification-submit"
            onClick={handleCustomSubmit}
            disabled={!customInput.trim()}
          >
            Send
          </button>
        </div>
      )}

      <div className="clarification-footer">
        <label className="clarification-remember">
          <input
            type="checkbox"
            checked={rememberForSession}
            onChange={(e) => setRememberForSession(e.target.checked)}
          />
          <span>Remember for this session</span>
        </label>

        {bestGuessAvailable && bestGuessValue && (
          <button
            className="clarification-best-guess"
            onClick={handleBestGuess}
          >
            Continue with best guess
          </button>
        )}

        <button className="clarification-dismiss" onClick={onDismiss}>
          Dismiss
        </button>
      </div>
    </div>
  );
}
