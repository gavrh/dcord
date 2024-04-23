use crate::utils::*;

pub struct Shard {
    pub token: String,
    pub intents: Vec<super::GatewayIntent>
}

impl Shard {

    pub async fn new(
        ws_url: Arc<Mutex<String>>,
        token: &str,
        intents: Vec<super::GatewayIntent>,
    ) -> Result<Shard, ()> {
        Ok(Shard {
            token: token.into(),
            intents
        })
    }

}