// dcrs - Discord API Library

#[derive(Debug)]
pub struct Channel {
    pub id: String,
    pub kind: ChannelKind,
    pub flags: Vec<ChannelFlag>
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ChannelKind {
    GUILD_TEXT          = 0,
    DM                  = 1,
    GUILD_VOICE         = 2,
    GROUP_DM            = 3,
    GUILD_CATEGORY      = 4,
    GUILD_ANNOUNCEMENT  = 5,
    ANNOUNCEMENT_THREAD = 10,
    PUBLIC_THREAD       = 11,
    PRIVATE_THREAD      = 12,
    GUILD_STAGE_VOICE   = 13,
    GUILD_DIRECTORY     = 14,
    GUILD_FORUM         = 15,
    GUILD_MEDIA         = 16
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ChannelFlag {
    PINNED                      = 1<<1,
    REQUIRE_TAG                 = 1<<4,
    HIDE_MEDIA_DOWNLOAD_OPTIONS = 1<<15
}

#[derive(Debug)]
pub struct ChannelManager<'a> {
    pub(crate) channels: std::collections::HashMap<String, &'a mut Channel>
}
impl<'a> ChannelManager<'a> {

    pub(crate) fn init() -> ChannelManager<'a> {
        return ChannelManager{ channels: std::collections::HashMap::new() }
    } 

    pub fn size(&self) -> usize {
        return self.channels.len();
    }

    pub fn add(&mut self, new_channel: &'a mut Channel) {
        self.channels.insert(new_channel.id.clone(), new_channel);
    }

    pub fn get(&self, channel_id: &str) -> Option<&Channel> {
        if let Some(channel) = self.channels.get(channel_id) {
            return Some(channel);
        }
        return None;
    }
    pub fn get_mut(&mut self, channel_id: &str) -> Option<&mut Channel> {
        if let Some(channel) = self.channels.get_mut(channel_id) {
            return Some(channel);
        }
        return None;
    }

}

#[derive(Debug)]
pub struct Client<'a> {
    pub channels: ChannelManager<'a>
}

impl<'a> Client<'a> {

    pub fn init() -> Client<'a> {
        return Client {
            channels: ChannelManager::init()
        }
    }

}

#[cfg(test)]
mod tests {}