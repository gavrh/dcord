mod event_emitter;

pub use event_emitter::*;
use futures::FutureExt;

use crate::utils::*;
use crate::gateway::{
    self, GatewayIntents, WsClient
};

use std::borrow::BorrowMut;
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
    connection: Option<Arc<Mutex<WsClient>>>
}
impl Client {

    pub fn builder(token: impl AsRef<str>, intents: GatewayIntents) -> ClientBuilder {
        ClientBuilder::new(token, intents)
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
            }
        });


        let _ = self.connection.as_ref().unwrap().lock().await.write(tokio_tungstenite::tungstenite::Message::Text(msg.to_string())).await;

        let conn = Arc::clone(&self.connection.as_ref().unwrap());

        tokio::spawn(async move {
            let heartbeat = serde_json::json!({
                "op": gateway::GatewayOpcode::Heartbeat, 
                "d": {}
            });
            loop {
                std::thread::sleep(tokio::time::Duration::from_millis(45000));
                let mut conn = conn.lock().await;
                println!("HERE FOR HEARTBEAT");
                if let Err(()) = conn.write(tokio_tungstenite::tungstenite::Message::Text(heartbeat.to_string())).await {
                    println!("ERROR");
                    return;
                } else {
                    println!("HEARTBEAT");
                }
                drop(conn);
            }
        });

        loop {
            let con = Arc::clone(&self.connection.as_ref().unwrap());
            let mut con = con.lock().await;
            match con.read() {
                Some(message) => {
                    tokio::spawn(async {
                        let rec_payload = 
                            serde_json::from_str::<gateway::WsRecPayload>(message.into_text().unwrap().as_str());
                        if rec_payload.is_err() { return }
                        println!("{:#?}", rec_payload.ok().unwrap());
                    });
                },
                _ => {}
            }
            drop(con);
        }

    }

}