pub mod manager;
pub mod session;
pub mod shell;

pub use manager::TerminalManager;
pub use session::{SessionInfo, TerminalSession};
pub use shell::{detect_shell, detect_shells};
