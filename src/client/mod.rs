mod event_emitter;

pub use event_emitter::EventHandler;

#[cfg(feature="cache")]
use crate::cache::CacheSettings;
use crate::gateway;
use std::sync::Arc;
use std::future::IntoFuture;
use futures::future::BoxFuture;

/// Builder implementing [`IntoFuture`] building a [`Client`] to interact with Discord.
#[cfg(feature="gateway")]
pub struct ClientBuilder {
    intents: gateway::GatewayIntents,
    event_handlers: Vec<Arc<dyn EventHandler>>,
    #[cfg(feature="cache")]
    cache_settings: CacheSettings,
    verbose: bool
}

#[cfg(feature="gateway")]
impl ClientBuilder {

    fn _new(intents: gateway::GatewayIntents) -> Self {
        Self {
            intents,
            event_handlers: vec![],
            #[cfg(feature="cache")]
            cache_settings: CacheSettings::default(),
            verbose: false
        }
    }

    pub fn new(_token: impl AsRef<str>, intents: gateway::GatewayIntents) -> Self {
        Self::_new(intents)
    }

    /// Enables verbose mode. Descriptive logging and debugging.
    pub fn verbose_mode(mut self) -> Self { 
        self.verbose = true;

        self
    }

    #[cfg(feature="cache")]
    pub fn cache_settings(mut self, settings: CacheSettings) -> Self {
        self.cache_settings = settings;

        self
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
        let intents = self.intents;
        let verbose = self.verbose;

        Box::pin(async move {
            let client = Client {
                _intents: intents,
                verbose
            };

            Ok(client)
        })
    }
}

/// Client Struct
#[derive(Debug)]
#[cfg(feature="gateway")]
pub struct Client {
    _intents: gateway::GatewayIntents,
    verbose: bool,
}

impl Client {

    pub fn builder(token: impl AsRef<str>, intents: gateway::GatewayIntents) -> ClientBuilder {
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

        if self.verbose {
            println!("Initializing shard info: {} - {}/{}", start_shard, init, total_shards);
        }
        if start_shard > 0 {
            return Err(());
        }

        Ok(())
    }

}