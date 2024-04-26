mod event_emitter;

pub use event_emitter::*;

use crate::utils::*;
use crate::gateway::{
    self, GatewayIntents, GatewayOpcode, GatewayEvent, WsClient
};

use std::future::IntoFuture;
use futures::future::BoxFuture;

/// [`Client`] builder.
pub struct ClientBuilder {
    intents: GatewayIntents,
    token: String,
}
impl ClientBuilder {

    pub(crate) fn _new(token: String, intents: GatewayIntents) -> Self {
        Self {
            intents,
            token
        }
    }

    pub fn new(token: impl AsRef<str>, intents: GatewayIntents) -> Self {
        Self::_new(token.as_ref().to_string(), intents)
    }

}
impl IntoFuture for ClientBuilder {
    
    type Output = Result<Client, ()>;
    type IntoFuture = BoxFuture<'static, Result<Client, ()>>;

    fn into_future(self) -> Self::IntoFuture {

        // client fields
        let intents = self.intents;
        let token = self.token;

        Box::pin(async move {

            let client = Client {
                intents,
                token,
                connection: None,
                last_heartbeat_sent: None,
                last_heartbeat_acknowledged: false
            };

            Ok(client)

        })

    }

}

/// Client
#[derive(Debug)]
#[allow(dead_code)]
pub struct Client {
    intents: GatewayIntents,
    token: String,
    connection: Option<Arc<Mutex<WsClient>>>,
    last_heartbeat_sent: Option<Instant>,
    last_heartbeat_acknowledged: bool,
}
impl Client {

    pub fn builder(token: impl AsRef<str>, intents: GatewayIntents) -> ClientBuilder {
        ClientBuilder::new(token, intents)
    }

    pub fn start_heartbeat(&mut self) {
        let conn = Arc::clone(self.connection.as_ref().unwrap());
        tokio::spawn(async move {
            let heartbeat = serde_json::json!({
                "op": gateway::GatewayOpcode::Heartbeat, 
                "d": {}
            });
            loop {
                std::thread::sleep(tokio::time::Duration::from_millis(40000));
                let mut conn = conn.lock().await;
                if let Err(()) = conn.write(tokio_tungstenite::tungstenite::Message::Text(heartbeat.to_string())).await {
                    drop(conn);
                    break
                }

                drop(conn);
            }
        });
    }

    pub async fn watch_gateway(&mut self) {

        let conn = Arc::clone(self.connection.as_ref().unwrap());
            loop {
                let mut conn = conn.lock().await;
                if let Some(msg) = conn.read() {
                    tokio::spawn(async move {
                        let payload = serde_json::from_str::<gateway::WsRecPayload>(msg.into_text().unwrap().as_str());
                        if payload.is_ok() {
                            let payload = payload.unwrap();
                            println!("{payload:#?}");
                            match payload.op {
                                GatewayOpcode::Dispatch => {
                                    if let Some(event) = payload.t {
                                        match event {
                                            _ => {}
                                        }
                                    }
                                },
                                GatewayOpcode::Heartbeat => {},
                                GatewayOpcode::Identify => {},
                                GatewayOpcode::PresenceUpdate => {},
                                GatewayOpcode::VoiceStateUpdate => {},
                                GatewayOpcode::Resume => {},
                                GatewayOpcode::Reconnect => {},
                                GatewayOpcode::RequestGuildMembers => {},
                                GatewayOpcode::InvalidSession => {},
                                GatewayOpcode::Hello => {},
                                GatewayOpcode::HeartbeatAck => {}
                            }
                        }
                });
                drop(conn);
            }
        }
    }

    pub async fn login(&mut self) -> Result<(), ()> {

        self.connection = Some(Arc::new(Mutex::new(WsClient::connect("wss://gateway.discord.gg").await.unwrap_or_else(|err| {
            panic!("{err:?}")
        }))));
        
        let msg = serde_json::json!({
            "op": gateway::GatewayOpcode::Identify,
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


        let _ = self.connection.as_ref().unwrap().lock().await.write(tokio_tungstenite::tungstenite::Message::Text(msg.to_string())).await;

        self.start_heartbeat();
        self.watch_gateway().await;

        Ok(())

    }

}