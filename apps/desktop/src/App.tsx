import { useState, useCallback, useEffect } from "react";
import { ActivityBar } from "./components/activitybar/ActivityBar";
import { Sidebar } from "./components/sidebar/Sidebar";
import { EditorArea } from "./components/editor/EditorArea";
import { Panel } from "./components/panel/Panel";
import { StatusBar } from "./components/statusbar/StatusBar";
import { TitleBar } from "./components/titlebar/TitleBar";
import { RightPanel } from "./components/right-panel/RightPanel";
import { CommandPalette } from "./components/common/CommandPalette";
import { SplashScreen } from "./components/common/SplashScreen";
import { SettingsPanel } from "./panels/settings/SettingsPanel";
import { OnboardingWizard } from "./ai/onboarding/OnboardingWizard";
import { useAppStore } from "./stores/appStore";

export default function App() {
  const {
    sidebarPanel,
    setSidebarPanel,
    bottomPanel,
    setBottomPanel,
    rightPanelVisible,
    settingsOpen,
    setSettingsOpen,
    commandPaletteOpen,
    setCommandPaletteOpen,
    splashVisible,
    setSplashVisible,
  } = useAppStore();

  const [showOnboarding, setShowOnboarding] = useState(false);
  const [onboarded, setOnboarded] = useState(() => {
    return localStorage.getItem("feverthoth-onboarded") === "true";
  });

  const handleOnboardingComplete = useCallback(() => {
    localStorage.setItem("feverthoth-onboarded", "true");
    setOnboarded(true);
    setShowOnboarding(false);
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "P") {
        e.preventDefault();
        setCommandPaletteOpen(!commandPaletteOpen);
      }
      if ((e.ctrlKey || e.metaKey) && e.key === ",") {
        e.preventDefault();
        setSettingsOpen(!settingsOpen);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [commandPaletteOpen, setCommandPaletteOpen, settingsOpen, setSettingsOpen]);

  if (splashVisible && onboarded) {
    return <SplashScreen onFinished={() => setSplashVisible(false)} />;
  }

  if (!onboarded || showOnboarding) {
    return <OnboardingWizard onComplete={handleOnboardingComplete} />;
  }

  return (
    <div className="app-container">
      <TitleBar />
      <div className="app-body">
        <ActivityBar
          activePanel={sidebarPanel}
          onPanelChange={setSidebarPanel}
        />
        <Sidebar activePanel={sidebarPanel} />
        <div className="main-content">
          <EditorArea />
          <Panel activePanel={bottomPanel} onPanelChange={setBottomPanel} />
        </div>
        {rightPanelVisible && <RightPanel />}
      </div>
      <StatusBar />

      {commandPaletteOpen && (
        <CommandPalette
          onClose={() => setCommandPaletteOpen(false)}
          onOpenSettings={() => setSettingsOpen(true)}
          onToggleTheme={() => {
            const current = document.documentElement.getAttribute("data-theme");
            document.documentElement.setAttribute(
              "data-theme",
              current === "frutiger-dark" ? "frutiger-light" : "frutiger-dark"
            );
          }}
          onNewFile={() => {}}
          onOpenFile={() => {}}
          onSwitchProvider={() => {}}
        />
      )}

      {settingsOpen && (
        <SettingsPanel onClose={() => setSettingsOpen(false)} />
      )}
    </div>
  );
}
