use serde::{Deserialize, Serialize};
use serde_json::{json};
use tokio_tungstenite::connect_async;
use uuid::Uuid;
use futures::{SinkExt, StreamExt};

// Main client struct
#[derive(Debug)]
pub struct SnapcastClient {
    url: String,
    pub status: Option<SnapcastStatus>,
}

// Response wrapper that matches the top-level JSON structure
#[derive(Debug, Deserialize)]
pub struct SnapcastResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: SnapcastStatus,
}

// Main status container
#[derive(Debug, Deserialize, Serialize)]
pub struct SnapcastStatus {
    pub server: ServerStatus,
}

// Server status containing streams and groups
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerStatus {
    pub server: ServerInfo,
    pub streams: Vec<Stream>,
    pub groups: Vec<Group>,
}

// Server information
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerInfo {
    pub host: HostInfo,
    pub snapserver: SnapserverInfo,
}

// Host information
#[derive(Debug, Deserialize, Serialize)]
pub struct HostInfo {
    pub name: String,
    pub os: String,
    pub arch: String,
    pub ip: String,
    pub mac: String,
}

// Snapserver information
#[derive(Debug, Deserialize, Serialize)]
pub struct SnapserverInfo {
    pub name: String,
    pub version: String,
    #[serde(rename = "controlProtocolVersion")]
    pub control_protocol_version: Option<u32>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: u32,
}

// Stream information
#[derive(Debug, Deserialize, Serialize)]
pub struct Stream {
    pub id: String,
    pub status: String,
    pub uri: Uri,
    pub properties: StreamProperties,
}

// Stream properties
#[derive(Debug, Deserialize, Serialize)]
pub struct StreamProperties {
    #[serde(rename = "canControl")]
    pub can_control: bool,
    #[serde(rename = "canPlay")]
    pub can_play: bool,
    #[serde(rename = "canPause")]
    pub can_pause: bool,
    #[serde(rename = "canSeek")]
    pub can_seek: bool,
    #[serde(rename = "canGoNext")]
    pub can_go_next: bool,
    #[serde(rename = "canGoPrevious")]
    pub can_go_previous: bool,
}

// URI information
#[derive(Debug, Deserialize, Serialize)]
pub struct Uri {
    pub path: String,
    pub scheme: String,
    pub query: Query,
    pub fragment: String,
    pub host: String,
    pub raw: String,
}

// URI query parameters
#[derive(Debug, Deserialize, Serialize)]
pub struct Query {
    pub name: String,
    #[serde(rename = "chunk_ms")]
    pub chunk_ms: Option<String>,
    pub codec: Option<String>,
    pub mode: Option<String>,
    #[serde(rename = "sampleformat")]
    pub sample_format: Option<String>,
}

// Group information
#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    #[serde(rename = "stream_id")]
    pub stream_id: String,
    pub muted: bool,
    pub clients: Vec<Client>,
}

// Client information
#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub id: String,
    #[serde(rename = "host")]
    pub host: HostInfo,
    #[serde(rename = "snapclient")]
    pub snapclient: SnapclientInfo,
    #[serde(rename = "config")]
    pub config: ClientConfig,
    pub connected: bool,
    #[serde(rename = "lastSeen")]
    pub last_seen: LastSeen,
}

// Client configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientConfig {
    pub instance: u32,
    pub latency: u32,
    pub name: String,
    pub volume: Volume,
}

// Volume information
#[derive(Debug, Deserialize, Serialize)]
pub struct Volume {
    pub muted: bool,
    pub percent: u32,
}

// Snapclient information
#[derive(Debug, Deserialize, Serialize)]
pub struct SnapclientInfo {
    pub name: String,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: u32,
    pub version: String,
}

// Last seen timestamp
#[derive(Debug, Deserialize, Serialize)]
pub struct LastSeen {
    pub sec: u64,
    pub usec: u64,
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
                    // First deserialize the full response
                    let response: SnapcastResponse = serde_json::from_str(&text)?;
                    self.status = Some(response.result);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}
