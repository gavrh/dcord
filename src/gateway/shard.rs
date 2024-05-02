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
    pub intents: GatewayIntents,
    count: u128,
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
                intents,
                count: 0,
            });
        }
        Err(())
    }

    pub async fn handle_heartbeat(&mut self) -> Result<(), ()> {
        let Some(heartbeat_interval) = self.heartbeat_interval else {
            return Ok(());
        };

        if let Some(last_sent) = self.last_heartbeat_sent {
            if last_sent.elapsed() <= heartbeat_interval {
                return Ok(());
            }
        };

        if !self.last_heartbeat_acknowledged {
            return Ok(());
        };

        // heartbeat
        let heartbeat_payload = serde_json::json!({
            "op": GatewayOpcode::Heartbeat,
            "d": self.seq
        });

        if let Err(_) = self
            .client
            .write(tungstenite::Message::Text(heartbeat_payload.to_string()))
            .await
        {
            return Err(());
        }

        self.last_heartbeat_sent = Some(Instant::now());
        self.last_heartbeat_acknowledged = false;

        Ok(())
    }

    pub async fn handle_payload(&mut self, payload: WsRecPayload) -> Result<(), ()> {
        // update sequence
        if let Some(s) = payload.s {
            self.seq = s;
        }

        print!("{:?}", payload.op);

        // match gateway opcode
        match payload.op {
            GatewayOpcode::Dispatch => {
                print!(" - {:?}", payload.t.as_ref().unwrap());

                match payload.t.unwrap() {
                    GatewayEvent::MessageCreate => {} // impl later
                    GatewayEvent::MessageDelete => {} // impl later
                    GatewayEvent::Resumed => {
                        print!("/ CONNECITON RESUMED");
                    }
                    _ => {}
                }

                match payload.d.unwrap() {
                    WsRecData::Ready { session_id } => {
                        self.session_id = Some(session_id);
                    }
                    _ => {}
                }
            }
            GatewayOpcode::Reconnect => {
                if let Err(()) = self.handle_reconnect_and_resume().await {
                    println!("ERROR RECONNECTING");
                    return Err(());
                }
            }
            GatewayOpcode::Hello => match payload.d.unwrap() {
                WsRecData::Heartbeat { heartbeat_interval } => {
                    self.heartbeat_interval =
                        Some(Duration::from_millis(heartbeat_interval - 5000));
                }
                _ => {}
            },
            GatewayOpcode::HeartbeatAck => {
                self.last_heartbeat_ack = Some(Instant::now());
                self.last_heartbeat_acknowledged = true;
            }
            _ => {}
        }

        println!();

        Ok(())
    }

    pub async fn handle_reconnect_and_resume(&mut self) -> Result<(), ()> {
        println!("RECONNECTING AND RESUMING!");

        self.client.close().await;
        self.client = WsClient::resume(
            crate::constants::GATEWAY,
            self.token.clone(),
            self.session_id.clone().unwrap(),
            self.seq,
        )
        .await?;

        Ok(())
    }

    /// Starts [`Shard`].
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
                        "name": "/legal",
                        "type": 3,
                    }],
                },
            }
        });

        if let Err(()) = self
            .client
            .write(tungstenite::Message::Text(msg.to_string()))
            .await
        {
            return Err(());
        }

        loop {
            // handle heartbeat
            if let Err(_) = self.handle_heartbeat().await {
                break;
            }

            if self.count % 100000000 == 0 && self.count >= 100000000 {
                println!(
                    "count {}/100000000 check: {:?}",
                    &self.count,
                    Instant::now()
                );
            }
            self.count += 1;

            // read websocket for new payload
            if let Ok(option_message) = self.client.read().await {
                match option_message {
                    Some(message) => {
                        if message.is_close() {
                            break;
                        }
                        let payload = serde_json::from_str::<WsRecPayload>(
                            message.into_text().unwrap().as_str(),
                        )
                        .unwrap_or_else(|err| {
                            panic!("{err:?}");
                        });

                        if let Err(_) = self.handle_payload(payload).await {
                            break;
                        }
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }

        Err(())
    }
}
