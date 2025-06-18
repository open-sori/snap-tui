use serde::Deserialize;
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;
use uuid::Uuid;
use futures::{SinkExt, StreamExt};

#[derive(Debug)]
pub struct SnapcastClient {
    url: String,
    pub status: Option<SnapcastStatus>,
}

#[derive(Debug, Deserialize)]
pub struct SnapcastStatus {
    pub server: ServerStatus,
}

#[derive(Debug, Deserialize)]
pub struct ServerStatus {
    pub server: ServerInfo,
    pub streams: Vec<Stream>,
}

#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    #[allow(dead_code)]
    pub host: HostInfo,
    pub snapserver: SnapserverInfo,
}

#[derive(Debug, Deserialize)]
pub struct HostInfo {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub os: String,
}

#[derive(Debug, Deserialize)]
pub struct SnapserverInfo {
    pub name: String,  // Now used in the UI
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Stream {
    #[allow(dead_code)]
    id: String,
    pub status: String,
    pub uri: Uri,
}

#[derive(Debug, Deserialize)]
pub struct Uri {
    pub path: String,
    pub scheme: String,
    pub query: Query,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub name: String,
    #[serde(rename = "chunk_ms")]
    pub chunk_ms: Option<String>,
    pub codec: Option<String>,
    pub mode: Option<String>,
    #[serde(rename = "sampleformat")]
    pub sample_format: Option<String>,
}

impl SnapcastClient {
    pub fn new(url: String) -> Self {
        SnapcastClient { url, status: None }
    }

    pub async fn fetch_status(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut write, mut read) = ws_stream.split();

        let request_id = Uuid::new_v4().to_string();
        let request = json!({
            "id": request_id,
            "jsonrpc": "2.0",
            "method": "Server.GetStatus"
        });

        write.send(tokio_tungstenite::tungstenite::Message::Text(request.to_string())).await?;

        if let Some(msg) = read.next().await {
            match msg? {
                tokio_tungstenite::tungstenite::Message::Text(text) => {
                    let response: Value = serde_json::from_str(&text)?;
                    if let Some(result) = response.get("result") {
                        self.status = Some(serde_json::from_value(result.clone())?);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}