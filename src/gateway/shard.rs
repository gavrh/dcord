use super::*;

use tokio_tungstenite::tungstenite;

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
    pub intents: GatewayIntents
}

impl Shard {

    pub async fn new(token: String, intents: GatewayIntents) -> Result<Self, ()> {

        if let Ok(client) = WsClient::connect("wss://gateway.discord.gg/").await {
            return Ok(Self {
                client,
                last_heartbeat_sent: Some(Instant::now()),
                last_heartbeat_ack: Some(Instant::now()),
                last_heartbeat_acknowledged: true,
                heartbeat_interval: Some(Duration::from_millis(40000)),
                seq: 0,
                session_id: None,
                started: Instant::now(),
                token,
                intents
            });
        }
        Err(())
    }

    pub async fn handle_heartbeat(&mut self) -> Result<(), ()> {
            
        if Instant::now().duration_since(self.last_heartbeat_ack.unwrap_or(Instant::now())) <= self.heartbeat_interval.unwrap() 
        && self.last_heartbeat_acknowledged {
            return Err(())
        }

        println!("SENDING HEARTBEAT");

        let heartbeat_payload = serde_json::json!({
            "op": GatewayOpcode::Heartbeat,
            "d": ""
        });

        let _ = self.client.write(tungstenite::Message::Text(heartbeat_payload.to_string())).await;
        self.last_heartbeat_sent = Some(Instant::now());
        self.last_heartbeat_acknowledged = false;

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), ()> {

        let msg = serde_json::json!({
            "op": GatewayOpcode::Identify,
            "d": {
                "token": self.token,
                "intents": self.intents.bitwise(),
                "properties": {
                    "os": std::env::consts::OS,
                    "browser": crate::constants::USER_AGENT,
                    "device": "github.com/grhx/dcrs"
                },
                "presence": {
                    "activities": [{
                        "name": "/help",
                        "type": 3,
                    }],
                },
            }
        });

        let _ = self.client.write(tungstenite::Message::Text(msg.to_string())).await;

        loop {

            // handle heartbeat
            let _ = self.handle_heartbeat().await;

            // read websocket for new payload
            match self.client.read() {
                Some(message) => {
                    
                    if message.is_close() { break }
                    let payload = serde_json::from_str::<WsRecPayload>(message.into_text().unwrap().as_str())
                    .unwrap_or_else(|err| { panic!("{err:?}"); });

                    println!("{payload:#?}");

                    // update sequence
                    if let Some(s) = payload.s {
                        self.seq = s;
                    }

                    // match gateway opcode
                    match payload.op { 
                        GatewayOpcode::HeartbeatAck => {
                            self.last_heartbeat_ack = Some(Instant::now());
                            self.last_heartbeat_acknowledged = true;
                        }
                        _ => {}
                    }

                },
                _ => {}
            }

        }

        Ok(())
    }

}