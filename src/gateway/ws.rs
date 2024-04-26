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

    pub async fn connect(url: &str) -> Result<Self, ()> {
        if let Ok(ws_conn) = connect_async(url).await {
            Ok(Self::new(ws_conn.0))
        } else { Err(()) }
    }
    
    pub async fn write(&mut self, msg: tungstenite::Message) -> Result<(), ()> {

        if let Err(why) = self.0.send(msg).await {
            println!("{why:?}");
            return Err(());
        } else { return Ok(()); }
    }

    pub fn read(&mut self) -> Option<tungstenite::Message> {
        let message = self.0.next().now_or_never();
        if let Some(res) = message {
            if let Some(res2) = res {
                if res2.is_err() { return None }
                Some(res2.unwrap())
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