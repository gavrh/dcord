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