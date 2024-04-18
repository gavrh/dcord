// dcrs - Discord API Library
pub mod channel;
pub mod user;

#[derive(Debug)]
pub struct Client<'a> {
    pub channels: channel::ChannelManager<'a>
}

impl<'a> Client<'a> {

    pub fn init() -> Client<'a> {
        return Client {
            channels: channel::ChannelManager::init()
        }
    }

}

#[cfg(test)]
mod tests {}