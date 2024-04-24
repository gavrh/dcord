use crate::utils::*;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    WebSocketStream,
    tungstenite
};
use futures::{SinkExt, StreamExt};

#[derive(Debug)]
pub struct WsClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WsClient {

    /// Initialize new [`WsClient`].
    pub fn new(ws_conn: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self(ws_conn)
    }

    /// Connect to gateway.
    pub async fn connect(url: &str) -> Self {
        Self::new(connect_async(url).await.unwrap().0)
    }

    /// Write to gateway.
    pub async fn write(&mut self, msg: tungstenite::Message) -> tungstenite::Result<(), ()> {

        if let Ok(()) = self.0.send(msg).await {
            Ok(())
        } else { Err(()) }

    }

    /// Read from gateway.
    pub async fn read(&mut self) -> Option<tungstenite::Message> {
        if let Some(msg) = self.0.next().await {
            let message = msg.expect("Error with message");
            Some(message)
        } else {
            None
        }
    }

    /// Close gateway connection.
    pub async fn close(&mut self) -> Result<(), ()> {
        if let Ok(()) = self.0.close(None).await {
            Ok(())
        } else { Err(()) }
    }

}

#[derive(Serialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum WsWriteData<'a> {
    Heartbeat(Option<u64>),
    Identify {
        compress: bool,
        token: &'a str,
        large_threshold: u8,
        shard: u16,
        intents: u64,
    },
    PresenceUpdate,
    VoiceStateUpdate,
    Resume {
        session_id: &'a str,
        token: &'a str,
        seq: u64
    },
    RequestGuildMembers
}

#[derive(Serialize)]
struct WsWritePayload<'a> {
    op: super::GatewayOpcode,
    d:  WsWriteData<'a>
}

#[derive(Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum WsReceiveData {
    Dipatch,
    Heartbeat,
    Reconnect,
    InvalidSession,
    Hello,
    HearbeatAck
}