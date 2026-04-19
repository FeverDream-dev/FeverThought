import { useEffect, useState } from "react";

interface SplashScreenProps {
  onFinished: () => void;
  minimumDuration?: number;
}

export function SplashScreen({ onFinished, minimumDuration = 1800 }: SplashScreenProps) {
  const [progress, setProgress] = useState(0);
  const [statusText, setStatusText] = useState("Initializing...");

  useEffect(() => {
    const steps = [
      { at: 10, text: "Loading engine..." },
      { at: 30, text: "Checking providers..." },
      { at: 50, text: "Loading workspace..." },
      { at: 70, text: "Preparing editor..." },
      { at: 90, text: "Almost ready..." },
    ];

    let elapsed = 0;
    const interval = setInterval(() => {
      elapsed += 50;
      const pct = Math.min(100, (elapsed / minimumDuration) * 100);
      setProgress(pct);

      for (const step of steps) {
        if (pct >= step.at) {
          setStatusText(step.text);
        }
      }

      if (pct >= 100) {
        clearInterval(interval);
        onFinished();
      }
    }, 50);

    return () => clearInterval(interval);
  }, [minimumDuration, onFinished]);

  return (
    <div className="splash-screen">
      <div className="splash-bg" />
      <div className="splash-content">
        <div className="splash-logo">
          <svg width="80" height="80" viewBox="0 0 80 80" fill="none">
            <defs>
              <linearGradient id="splash-grad" x1="0" y1="0" x2="80" y2="80">
                <stop stopColor="#4fc3f7" />
                <stop offset="0.5" stopColor="#2196f3" />
                <stop offset="1" stopColor="#1976d2" />
              </linearGradient>
            </defs>
            <circle cx="40" cy="40" r="36" fill="url(#splash-grad)" />
            <text x="40" y="48" textAnchor="middle" fill="white" fontSize="28" fontWeight="bold">FT</text>
          </svg>
        </div>
        <h1 className="splash-title">FeverThoth IDE</h1>
        <p className="splash-subtitle">AI-first coding environment</p>
        <div className="splash-progress-track">
          <div className="splash-progress-fill" style={{ width: `${progress}%` }} />
        </div>
        <p className="splash-status">{statusText}</p>
      </div>
    </div>
  );
}
