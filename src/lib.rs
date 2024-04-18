// dcrs - Discord API Library

#[derive(Debug)]
pub struct Channel {
    pub id: String,
    pub kind: ChannelKind,
    pub flags: Vec<ChannelFlag>
}

#[derive(Debug)]
pub enum ChannelKind {
    GuildText           = 0,
    Dm                  = 1,
    GuildVoice          = 2,
    GroupDm             = 3,
    GuildCategory       = 4,
    GuildAnnouncement   = 5,
    AnnouncementThread  = 10,
    PublicThread        = 11,
    PrivateThread       = 12,
    GuildStageVoice     = 13,
    GuildDirectory      = 14,
    GuildForum          = 15,
    GuildMedia          = 16
}

#[derive(Debug)]
pub enum ChannelFlag {
    Pinned                      = 1<<1,
    RequireTag                  = 1<<4,
    HideMediaDownloadOptions    = 1<<15
}

#[derive(Debug)]
pub struct ChannelManager<'a> {
    channels: std::collections::HashMap<String, &'a mut Channel>
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
        match self.channels.get(channel_id) {
            Some(channel) => return Some(channel),
            _ => return None
        }
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