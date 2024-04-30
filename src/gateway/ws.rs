use super::*;
use crate::utils::*;

use tokio_tungstenite::{
    WebSocketStream,
    MaybeTlsStream,
    connect_async,
    tungstenite,
};
use tokio::net::TcpStream;
use futures::{
    stream::FusedStream, FutureExt, SinkExt, StreamExt
};


#[derive(Debug)]
pub struct WsClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WsClient {

    /// Returns new [`WsClient`].
    pub fn new(ws_conn: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self(ws_conn)
    }

    /// Connect to gateway.
    /// 
    /// Returns a [`WsClient`]
    pub async fn connect(url: &str) -> Result<Self, ()> {
        if let Ok(ws_conn) = connect_async(url).await {
            Ok(Self::new(ws_conn.0))
        } else { Err(()) }
    }

    /// Re-connects to gateway.
    /// 
    /// Returns a [`WsClient`]
    pub async fn resume(url: &str, token: String, session_id: String, seq: u64) -> Result<Self, ()> {

        if let Ok(ws_conn) = connect_async(url).await {
            let mut conn = Self::new(ws_conn.0);

            let resume_msg = serde_json::json!({
                "op": GatewayOpcode::Resume,
                "d": {
                    "token": token,
                    "session_id": session_id,
                    "seq": seq,
                }
            });

            if let Err(err) = conn.write(tungstenite::Message::Text(resume_msg.to_string())).await {
                println!("ERROR RESUMING {err:?}");
            }

            Ok(conn)

        } else { Err(()) }

    }

    /// Send message to gateway connection.
    pub async fn write(&mut self, msg: tungstenite::Message) -> Result<(), ()> {

        if let Err(why) = self.0.send(msg).await {
            println!("{why:?}");
            return Err(());
        } else { return Ok(()); }
    }

    /// Check gateway connection for next message.
    pub async fn read(&mut self) -> Result<Option<tungstenite::Message>, ()> {
        if let Some(next) = self.0.next().now_or_never() {
            if let Some(res) = next {
                if let Ok(message) = res {
                    Ok(Some(message))
                } else {
                    Err(())
                }
            } else { Ok(None) }
        } else { Ok(None) }
    }

    /// Check if gateway connection is closed.
    pub fn is_closed(&self) -> bool {
        self.0.is_terminated()
    }

    pub async fn close(&mut self) {
        let _ = self.0.close(None).await;
    }

}

#[derive(Debug)]
#[allow(dead_code)]
enum WsWriteData {
    Identify,
    Heartbeat(Option<u64>)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(untagged)]
pub enum WsRecData {
    Ready {
        session_id: String,
    },
    Heartbeat {
        heartbeat_interval: u64,
    },
    None {}
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WsRecPayload {
    pub op: GatewayOpcode,
    pub s: Option<u64>,
    pub t: Option<GatewayEvent>,
    pub d: Option<WsRecData>
}