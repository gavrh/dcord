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
                last_heartbeat_sent: None,
                last_heartbeat_ack: None,
                last_heartbeat_acknowledged: true,
                heartbeat_interval: None,
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
            
        let Some(heartbeat_interval) = self.heartbeat_interval else {
            return Err(())
        };

        if let Some(last_sent) = self.last_heartbeat_sent {
            if last_sent.elapsed() <= heartbeat_interval {
                return Err(())
            }
        };

        if !self.last_heartbeat_acknowledged {
            return Err(())
        };

        // heartbeat
        let heartbeat_payload = serde_json::json!({
            "op": GatewayOpcode::Heartbeat,
            "d": ""
        });

        self.client.write(tungstenite::Message::Text(heartbeat_payload.to_string())).await?;
        self.last_heartbeat_sent = Some(Instant::now());
        self.last_heartbeat_acknowledged = false;

        Ok(())
    }

    pub async fn handle_reconnect_and_resume(&mut self) -> Result<(), ()> {

        println!("RECONNECTING AND RESUMING!");

        self.client.close().await;
        self.client = WsClient::connect("wss://gateway.discord.gg/").await?;

        let resume_msg = serde_json::json!({
            "op": GatewayOpcode::Resume,
            "d": {
                "token": self.token,
                "session_id": self.session_id.as_ref().unwrap(),
                "seq": self.seq
            }
        });

        self.client.write(tungstenite::Message::Text(resume_msg.to_string())).await?;

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
                    "device": crate::constants::GITHUB
                },
                "presence": {
                    "activities": [{
                        "name": "/help",
                        "type": 3,
                    }],
                },
            }
        });

        self.client.write(tungstenite::Message::Text(msg.to_string()))
        .await.unwrap_or_else(|_| { panic!("ERROR IDENTIFYING")});

        loop {

            // handle heartbeat
            let _ = self.handle_heartbeat().await;

            // read websocket for new payload
            match self.client.read() {
                Some(message) => {
                    
                    if message.is_close() { break }
                    let payload = serde_json::from_str::<WsRecPayload>(message.into_text().unwrap().as_str())
                    .unwrap_or_else(|err| { panic!("{err:?}"); });

                    // println!("{payload:#?}");

                    // update sequence
                    if let Some(s) = payload.s {
                        self.seq = s;
                    }

                    // match gateway opcode
                    match payload.op { 
                        GatewayOpcode::Dispatch => {

                            println!("{:?}", payload.t.unwrap());
                            
                            match payload.d.unwrap() {
                                WsRecData::Ready { session_id } => {
                                    self.session_id = Some(session_id);
                                }
                                _ => {} 
                            }

                        },
                        GatewayOpcode::Reconnect => {
                            self.handle_reconnect_and_resume().await?;
                        },
                        GatewayOpcode::Hello => {
                            match payload.d.unwrap() {
                                WsRecData::Heartbeat { heartbeat_interval } => {
                                    self.heartbeat_interval = Some(Duration::from_millis(heartbeat_interval));
                                },
                                _ => {}
                            }
                        },
                        GatewayOpcode::HeartbeatAck => {
                            self.last_heartbeat_ack = Some(Instant::now());
                            self.last_heartbeat_acknowledged = true;
                        },
                        _ => {}
                    }

                },
                _ => {}
            }

        }

        Ok(())
    }

}