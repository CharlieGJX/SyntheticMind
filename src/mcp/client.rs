use anyhow::{anyhow, Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
    pub id: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<Value>,
    pub id: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

impl McpTool {
    pub fn to_declaration(&self, server_name: &str) -> crate::function::FunctionDeclaration {
        let parameters: crate::function::JsonSchema = serde_json::from_value(self.input_schema.clone())
            .unwrap_or_else(|_| crate::function::JsonSchema {
                type_value: Some("object".into()),
                description: None,
                properties: Some(indexmap::IndexMap::new()),
                items: None,
                any_of: None,
                enum_value: None,
                default: None,
                required: None,
            });

        crate::function::FunctionDeclaration {
            name: format!("mcp__{}__{}", server_name, self.name),
            description: self.description.clone(),
            parameters,
            agent: false,
        }
    }
}

#[derive(Debug)]
pub struct McpClient {
    #[allow(dead_code)]
    child: Child,
    request_tx: mpsc::UnboundedSender<(JsonRpcRequest, oneshot::Sender<Result<Value>>)>,
}

impl McpClient {
    pub fn spawn(command: &str, args: &[String], envs: &HashMap<String, String>) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .envs(envs)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| format!("Failed to spawn MCP server: {}", command))?;

        let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout"))?;

        let (request_tx, mut request_rx) = mpsc::unbounded_channel::<(JsonRpcRequest, oneshot::Sender<Result<Value>>)>();

        // Background task to handle communication
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            let mut pending_requests: HashMap<String, oneshot::Sender<Result<Value>>> = HashMap::new();

            loop {
                tokio::select! {
                    // Send requests to the server
                    req_data = request_rx.recv() => {
                        if let Some((req, tx)) = req_data {
                            let id = req.id.as_str().unwrap_or_default().to_string();
                            pending_requests.insert(id, tx);
                            let json = match serde_json::to_string(&req) {
                                Ok(j) => j,
                                Err(e) => {
                                    if let Some(tx) = pending_requests.remove(&req.id.as_str().unwrap_or_default().to_string()) {
                                        let _ = tx.send(Err(anyhow!("Failed to serialize request: {}", e)));
                                    }
                                    continue;
                                }
                            };
                            if let Err(e) = stdin.write_all(format!("{}\n", json).as_bytes()).await {
                                error!("Failed to write to MCP server stdin: {}", e);
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    // Receive responses from the server
                    line_res = reader.next_line() => {
                        match line_res {
                            Ok(Some(line)) => {
                                if let Ok(res) = serde_json::from_str::<JsonRpcResponse>(&line) {
                                    let id = match &res.id {
                                        Value::String(s) => s.clone(),
                                        Value::Number(n) => n.to_string(),
                                        _ => continue,
                                    };
                                    if let Some(tx) = pending_requests.remove(&id) {
                                        if let Some(error) = res.error {
                                            let _ = tx.send(Err(anyhow!("MCP Error: {}", error)));
                                        } else {
                                            let _ = tx.send(Ok(res.result.unwrap_or(Value::Null)));
                                        }
                                    }
                                }
                            }
                            Ok(None) => break, // EOF
                            Err(e) => {
                                error!("Error reading from MCP server: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(Self { child, request_tx })
    }

    pub async fn call(&self, method: &str, params: Value) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params,
            id: json!(id),
        };

        self.request_tx.send((req, tx)).map_err(|_| anyhow!("Failed to send request to channel"))?;
        rx.await.map_err(|_| anyhow!("Failed to receive response from channel"))?
    }

    pub async fn initialize(&self) -> Result<Value> {
        self.call("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "vibe-ai",
                "version": env!("CARGO_PKG_VERSION")
            }
        })).await
    }

    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let res = self.call("tools/list", json!({})).await?;
        let tools: Vec<McpTool> = serde_json::from_value(res["tools"].clone())
            .context("Failed to parse tools from MCP server")?;
        Ok(tools)
    }
}
