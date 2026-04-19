use lsp_types::*;
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticEvent {
    pub uri: String,
    pub diagnostics: Vec<SimpleDiagnostic>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleDiagnostic {
    pub message: String,
    pub severity: String,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub source: Option<String>,
}

impl SimpleDiagnostic {
    pub fn from_lsp(d: &Diagnostic) -> Self {
        Self {
            message: d.message.clone(),
            severity: match d.severity {
                Some(DiagnosticSeverity::ERROR) => "error".into(),
                Some(DiagnosticSeverity::WARNING) => "warning".into(),
                Some(DiagnosticSeverity::INFORMATION) => "info".into(),
                Some(DiagnosticSeverity::HINT) => "hint".into(),
                _ => "info".into(),
            },
            start_line: d.range.start.line,
            start_col: d.range.start.character,
            end_line: d.range.end.line,
            end_col: d.range.end.character,
            source: d.source.clone(),
        }
    }
}

pub struct LspHandlers;

impl LspHandlers {
    pub fn handle_notification(method: &str, params: Value) -> Option<DiagnosticEvent> {
        match method {
            "textDocument/publishDiagnostics" => {
                let typed: Result<PublishDiagnosticsParams, _> = serde_json::from_value(params);
                typed.ok().map(|p| DiagnosticEvent {
                    uri: p.uri.to_string(),
                    diagnostics: p
                        .diagnostics
                        .iter()
                        .map(SimpleDiagnostic::from_lsp)
                        .collect(),
                })
            }
            _ => None,
        }
    }
}
