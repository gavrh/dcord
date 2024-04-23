mod data_manager;
mod event_emitter;

pub use data_manager::*;
pub use event_emitter::EventHandler;

use crate::gateway;
use std::sync::Arc;
use std::future::IntoFuture;
use futures::future::BoxFuture;

pub struct ClientBuilder {
    event_handlers: Vec<Arc<dyn EventHandler>>
}

impl ClientBuilder {

    fn _new() -> Self {
        Self {
            event_handlers: vec![]
        }
    }

    pub fn new(_token: impl AsRef<str>, _intents: Vec<gateway::GatewayIntent>) -> Self {
        Self::_new()
    }

    /// Adds an event handler with multiple methods for each possible event.
    pub fn event_handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handlers.push(Arc::new(event_handler));

        self
    }

    /// Adds an event handler with multiple methods for each possible event.
    /// 
    /// Passed by Arc.
    pub fn event_handler_arc<H: EventHandler + 'static>(mut self, event_handler_arc: Arc<H>) -> Self {
        self.event_handlers.push(event_handler_arc);

        self
    }

}

#[cfg(feature="gateway")]
impl IntoFuture for ClientBuilder {
    type Output = Result<Client, ()>;
    type IntoFuture = BoxFuture<'static, Result<Client, ()>>;

    fn into_future(self) -> Self::IntoFuture {
        // let event_handlers = self.event_handlers;

        Box::pin(async move {
            let client = Client {};

            Ok(client)
        })
    }
}

/// Client Struct
#[derive(Debug)]
pub struct Client {}

impl Client {

    pub fn builder(token: impl AsRef<str>, intents: Vec<gateway::GatewayIntent>) -> ClientBuilder {
        ClientBuilder::new(token, intents)
    }


    pub async fn start(&mut self) -> Result<(), ()> {
        self.start_connection(0, 0, 1).await
    }

    async fn start_connection(
        &mut self,
        start_shard: u32,
        end_shard: u32,
        total_shards: u32,
    ) -> Result<(), ()> {

        let init = end_shard-start_shard+1;
        println!("Initializing shard info: {} - {}/{}", start_shard, init, total_shards);

        if start_shard > 0 {
            return Err(());
        }

        Ok(())
    }

}