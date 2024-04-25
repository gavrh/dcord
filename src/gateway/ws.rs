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
    SinkExt,
    StreamExt,
};


#[derive(Debug)]
pub struct WsClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WsClient {

    pub fn new(ws_conn: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self(ws_conn)
    }

    pub async fn connect(url: &str) -> Self {
        Self::new(connect_async(url).await.unwrap().0)
    }
    
    pub async fn write(&mut self, msg: tungstenite::Message) -> Result<(), ()> {
        if let Ok(()) = self.0.send(msg).await {
            Ok(())
        } else { Err(()) }
    }

    pub async fn read(&mut self) -> Option<tungstenite::Message> {
        if let Some(msg) = self.0.next().await {
            let message = msg.unwrap();
            Some(message)
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