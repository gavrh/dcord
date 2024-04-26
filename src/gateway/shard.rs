use super::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Shard {
    pub client: WsClient,
    last_heartbeat_sent: Option<Instant>,
    last_heartbeat_ack: Option<Instant>,
    last_heartbeat_acknowledged: bool,
    heartbeat_interval: Option<Duration>,
    seq: u64,
    session_id: Option<String>,
    pub started: Instant,
    pub token: String,
    pub intents: GatewayIntent
}