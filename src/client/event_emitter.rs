use async_trait::async_trait;

#[async_trait]
#[allow(unused_variables)]
pub trait EventHandler: Send + Sync {
    /// Dispatched when a message is created.
    /// 
    /// Provides the message's data.
    async fn message_create(&self, message: String){}
}