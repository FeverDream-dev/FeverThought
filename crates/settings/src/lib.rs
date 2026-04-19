use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub editor: EditorSettings,
    pub terminal: TerminalSettings,
    pub ai: AiSettings,
    pub appearance: AppearanceSettings,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub mode: AppMode,
    pub auto_save: bool,
    pub auto_save_delay_ms: u64,
    pub recent_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: u32,
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub minimap_enabled: bool,
    pub line_numbers: bool,
    pub bracket_pair_colorization: bool,
    pub format_on_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub shell: Option<String>,
    pub font_family: String,
    pub font_size: u32,
    pub cursor_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub default_provider: String,
    pub ollama_url: String,
    pub ollama_port: u16,
    pub default_model: String,
    pub vision_model: String,
    pub plan_before_edit: bool,
    pub privacy_mode: PrivacyMode,
    pub allow_cloud_screenshots: bool,
    pub providers: HashMap<String, ProviderSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSettings {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,
    pub accent_color: String,
    pub transparency: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AppMode {
    Simple,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyMode {
    Strict,
    Balanced,
    Open,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                mode: AppMode::Simple,
                auto_save: true,
                auto_save_delay_ms: 1000,
                recent_projects: Vec::new(),
            },
            editor: EditorSettings {
                font_family: "Fira Code".into(),
                font_size: 14,
                tab_size: 4,
                insert_spaces: true,
                word_wrap: false,
                minimap_enabled: true,
                line_numbers: true,
                bracket_pair_colorization: true,
                format_on_save: false,
            },
            terminal: TerminalSettings {
                shell: None,
                font_family: "Fira Code".into(),
                font_size: 14,
                cursor_style: "block".into(),
            },
            ai: AiSettings {
                default_provider: "ollama".into(),
                ollama_url: "http://localhost".into(),
                ollama_port: 11434,
                default_model: "llama3.2".into(),
                vision_model: "qwen2.5vl".into(),
                plan_before_edit: true,
                privacy_mode: PrivacyMode::Balanced,
                allow_cloud_screenshots: false,
                providers: HashMap::new(),
            },
            appearance: AppearanceSettings {
                theme: "frutiger-light".into(),
                accent_color: "#0078d4".into(),
                transparency: true,
            },
            extra: HashMap::new(),
        }
    }
}

pub struct SettingsManager {
    settings: Settings,
    path: PathBuf,
}

impl SettingsManager {
    pub fn load() -> anyhow::Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("feverthoth");

        std::fs::create_dir_all(&config_dir)?;
        let path = config_dir.join("settings.json");

        let settings = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Settings::default()
        };

        Ok(Self { settings, path })
    }

    pub fn get(&self) -> &Settings {
        &self.settings
    }

    pub fn update(&mut self, settings: Settings) -> anyhow::Result<()> {
        self.settings = settings;
        self.save()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(&self.settings)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }
}
