mod client;

pub use client::{McpClient, McpTool};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct McpServerManager {
    clients: Arc<Mutex<HashMap<String, McpClient>>>,
}

impl McpServerManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, name: &str, client: McpClient) {
        let mut clients = self.clients.lock().await;
        clients.insert(name.to_string(), client);
    }

    pub async fn list_all_tools(&self) -> Result<Vec<(String, Vec<McpTool>)>> {
        let mut all_tools = Vec::new();
        let clients = self.clients.lock().await;
        for (name, client) in clients.iter() {
            let tools = client.list_tools().await?;
            all_tools.push((name.clone(), tools));
        }
        Ok(all_tools)
    }

    pub async fn call_tool(&self, server_name: &str, tool_name: &str, arguments: serde_json::Value) -> Result<serde_json::Value> {
        let clients = self.clients.lock().await;
        let client = clients.get(server_name).ok_or_else(|| anyhow::anyhow!("MCP server not found: {}", server_name))?;
        client.call("tools/call", serde_json::json!({
            "name": tool_name,
            "arguments": arguments
        })).await
    }
}
