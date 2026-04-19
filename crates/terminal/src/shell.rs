use std::path::Path;

pub fn detect_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "cmd.exe".to_string()
        } else {
            "/bin/sh".to_string()
        }
    })
}

pub fn detect_shells() -> Vec<String> {
    let mut shells = Vec::new();
    let candidates = [
        "/bin/zsh",
        "/bin/bash",
        "/bin/sh",
        "/usr/bin/fish",
        "/usr/bin/tcsh",
        "/usr/bin/csh",
    ];

    for path in &candidates {
        if Path::new(path).exists() {
            shells.push(path.to_string());
        }
    }

    if shells.is_empty() {
        shells.push(detect_shell());
    }

    shells
}
