use crate::utils::*;

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Shard {
    pub client: super::WsClient,
    last_heartbeat_sent: Option<Instant>,
    last_heartbeat_ack: Option<Instant>,
    heartbeat_interval: Option<Duration>,
    pub started: Instant,
    pub token: String,
    pub intents: Vec<super::GatewayIntent>
}

impl Shard {

    pub async fn new(
        ws_url: Arc<Mutex<String>>,
        token: &str,
        intents: Vec<super::GatewayIntent>
    ) -> Result<Shard, ()> {

        let url = ws_url.lock().await.clone();
        let client = connect(&url).await;
        let last_heartbeat_sent = None;
        let last_heartbeat_ack = None;
        let heartbeat_interval = None;

        Ok(Shard {
            client,
            last_heartbeat_sent,
            last_heartbeat_ack,
            heartbeat_interval,
            started: Instant::now(),
            token: token.into(),
            intents
        })
    }

}
pub async fn connect(url: &str) -> super::WsClient {
    super::WsClient::connect(url).await
}