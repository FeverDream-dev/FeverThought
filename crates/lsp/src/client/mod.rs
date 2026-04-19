use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use lsp_types::*;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::oneshot;
use tracing::{error, info};

type PendingRequests = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Value, String>>>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspNotification {
    pub method: String,
    pub params: Value,
}

pub struct LspClient {
    language_id: String,
    request_counter: Arc<AtomicU64>,
    stdin_tx: Option<std::sync::mpsc::Sender<String>>,
    pending: PendingRequests,
    capabilities: Arc<Mutex<Option<ServerCapabilities>>>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
    #[allow(dead_code)]
    notification_tx: Option<std::sync::mpsc::Sender<LspNotification>>,
}

impl LspClient {
    pub fn language_id(&self) -> &str {
        &self.language_id
    }

    pub fn capabilities(&self) -> Option<ServerCapabilities> {
        self.capabilities.lock().clone()
    }

    pub fn start(
        language_id: String,
        command: &str,
        args: &[String],
        _root_uri: lsp_types::Uri,
    ) -> anyhow::Result<(Self, Child)> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn LSP server '{}': {}", command, e))?;

        let stdin = child.stdin.take().expect("stdin not captured");
        let stdout = child.stdout.take().expect("stdout not captured");

        let pending: PendingRequests = Arc::new(Mutex::new(HashMap::new()));
        let capabilities = Arc::new(Mutex::new(None));
        let is_running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let request_counter = Arc::new(AtomicU64::new(1));

        let (stdin_tx, stdin_rx) = std::sync::mpsc::channel::<String>();

        let pending_clone = pending.clone();
        let caps_clone = capabilities.clone();
        let running_clone = is_running.clone();

        let client = Self {
            language_id,
            request_counter,
            stdin_tx: Some(stdin_tx),
            pending,
            capabilities,
            is_running,
            notification_tx: None,
        };

        std::thread::spawn(move || {
            let (notification_tx, _notification_rx) = std::sync::mpsc::channel::<LspNotification>();
            read_lsp_stdout(
                stdout,
                pending_clone,
                caps_clone,
                running_clone,
                Some(notification_tx),
            );
        });

        std::thread::spawn(move || {
            write_lsp_stdin(stdin, stdin_rx);
        });

        Ok((client, child))
    }

    pub async fn request<R: lsp_types::request::Request>(
        &self,
        params: R::Params,
    ) -> anyhow::Result<R::Result>
    where
        R::Params: Serialize,
        R::Result: for<'de> Deserialize<'de>,
    {
        let id = self.request_counter.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": R::METHOD,
            "params": params,
        });

        let msg = format!(
            "Content-Length: {}\r\n\r\n{}",
            request.to_string().len(),
            request
        );
        if let Some(ref stdin_tx) = self.stdin_tx {
            stdin_tx
                .send(msg)
                .map_err(|e| anyhow::anyhow!("Failed to send LSP request: {}", e))?;
        }

        let response = rx
            .await
            .map_err(|_| anyhow::anyhow!("LSP request cancelled"))?
            .map_err(|e| anyhow::anyhow!("LSP error: {}", e))?;
        let result: R::Result = serde_json::from_value(response)?;
        Ok(result)
    }

    pub fn notify<N: lsp_types::notification::Notification>(
        &self,
        params: N::Params,
    ) -> anyhow::Result<()>
    where
        N::Params: Serialize,
    {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": N::METHOD,
            "params": params,
        });
        let msg = format!(
            "Content-Length: {}\r\n\r\n{}",
            notification.to_string().len(),
            notification
        );
        if let Some(ref stdin_tx) = self.stdin_tx {
            stdin_tx
                .send(msg)
                .map_err(|e| anyhow::anyhow!("Failed to send LSP notification: {}", e))?;
        }
        Ok(())
    }

    pub fn notifications(&self) -> Option<&std::sync::mpsc::Receiver<LspNotification>> {
        None
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

fn write_lsp_stdin(mut stdin: ChildStdin, rx: std::sync::mpsc::Receiver<String>) {
    while let Ok(msg) = rx.recv() {
        if stdin.write_all(msg.as_bytes()).is_err() {
            break;
        }
        if stdin.flush().is_err() {
            break;
        }
    }
}

fn read_lsp_stdout(
    stdout: ChildStdout,
    pending: PendingRequests,
    capabilities: Arc<Mutex<Option<ServerCapabilities>>>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
    notification_tx: Option<std::sync::mpsc::Sender<LspNotification>>,
) {
    let mut reader = BufReader::new(stdout);
    let mut header_buf = String::new();

    loop {
        header_buf.clear();
        let mut content_length: Option<usize> = None;

        loop {
            header_buf.clear();
            if reader.read_line(&mut header_buf).unwrap_or(0) == 0 {
                is_running.store(false, Ordering::SeqCst);
                return;
            }
            let line = header_buf.trim();
            if line.is_empty() {
                break;
            }
            if let Some(len_str) = line.strip_prefix("Content-Length: ") {
                content_length = len_str.trim().parse().ok();
            }
        }

        let Some(length) = content_length else {
            continue;
        };

        let mut body_buf = vec![0u8; length];
        if reader.read_exact(&mut body_buf).is_err() {
            is_running.store(false, Ordering::SeqCst);
            return;
        }

        let body: Value = match serde_json::from_slice(&body_buf) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse LSP message: {}", e);
                continue;
            }
        };

        if let Some(id) = body.get("id").and_then(|v| v.as_u64()) {
            let result = if let Some(error) = body.get("error") {
                Err(error.to_string())
            } else {
                Ok(body.get("result").cloned().unwrap_or(Value::Null))
            };

            if let Some(sender) = pending.lock().remove(&id) {
                let _ = sender.send(result);
            }

            if let Some(result) = body.get("result") {
                if let Ok(caps) = serde_json::from_value::<InitializeResult>(result.clone()) {
                    *capabilities.lock() = Some(caps.capabilities);
                    info!("LSP server initialized with capabilities");
                }
            }
        } else if let Some(method) = body.get("method").and_then(|v| v.as_str()) {
            let params = body
                .get("params")
                .cloned()
                .unwrap_or(Value::Object(serde_json::Map::new()));
            if let Some(ref tx) = notification_tx {
                let _ = tx.send(LspNotification {
                    method: method.to_string(),
                    params,
                });
            }
        }
    }
}
