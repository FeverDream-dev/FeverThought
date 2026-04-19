use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("LSP error: {0}")]
    Lsp(String),
    #[error("Workspace error: {0}")]
    Workspace(String),
    #[error("Settings error: {0}")]
    Settings(String),
    #[error("Security error: {0}")]
    Security(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    #[error("Cancelled")]
    Cancelled,
}

pub type CoreResult<T> = Result<T, CoreError>;
