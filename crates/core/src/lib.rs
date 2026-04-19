use std::sync::Arc;

pub mod error;
pub mod state;

pub use error::{CoreError, CoreResult};
pub use state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppMode {
    Simple,
    Advanced,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub mode: String,
}

pub fn app_info() -> AppInfo {
    AppInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        mode: "simple".to_string(),
    }
}
