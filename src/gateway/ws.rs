use std::future::IntoFuture;

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
    FutureExt, SinkExt, StreamExt
};


#[derive(Debug)]
pub struct WsClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WsClient {

    pub fn new(ws_conn: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self(ws_conn)
    }

    /// Connect to websocket.
    /// 
    /// Returns a [`WsClient`]
    pub async fn connect(url: &str) -> Result<Self, ()> {
        if let Ok(ws_conn) = connect_async(url).await {
            Ok(Self::new(ws_conn.0))
        } else { Err(()) }
    }

    /// Send message to websocket connection.
    pub async fn write(&mut self, msg: tungstenite::Message) -> Result<(), ()> {

        if let Err(why) = self.0.send(msg).await {
            println!("{why:?}");
            return Err(());
        } else { return Ok(()); }
    }

    /// Check websocket connection for next message.
    pub fn read(&mut self) -> Option<tungstenite::Message> {
        if let Some(next) = self.0.next().now_or_never() {
            if let Some(res) = next {
                if res.is_ok() {
                    Some(res.unwrap())
                } else { None }
            } else { None }
        } else { None }
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
enum WsRecData {
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WsRecPayload {
    op: GatewayOpcode,
    s: Option<u64>,
    t: Option<GatewayEvent>,
}