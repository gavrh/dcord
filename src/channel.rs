use crate::guild;
use crate::channel;
use crate::user;

///
/// [discord docs @ channel-object](https://discord.com/developers/docs/resources/channel#channel-object)
#[derive(Debug)]
pub struct Channel<'a> {
    pub id:                     String,
    pub kind:                   ChannelKind,
    pub guild:                  &'a guild::Guild<'a>,
    /// sorting position of the channel
    pub position:               u32,

    // permission_overwrites

    /// the name of the channel (1-100 characters)
    pub name:                   String,
    /// channel topic (0-4096 characters for `GUILD_FORUM` and `GUILD_MEDIA` channels, 0-1024 characters for all others)
    pub topic:                  String,
    /// wether the channel is nsfw
    pub nsfw:                   bool,
    /// bitrate (in bits) of `GUILD_VOICE` channel
    pub bitrate:                u32,
    /// user limit of `GUILD_VOICE` channel
    pub user_limit:             u32,
    /// amount of seconds a user has to wait before sending another message (0-21600);
    /// bots, as well as users with the permission `MANAGE_MESSAGES` or `MANAGE_CHANNEL`, are unaffected
    pub rate_limit:             u32,
    /// recipients of `DM` or `GROUP_DM` channel
    pub recipients:             Vec<&'a user::User>,
    /// icon hash of `GROUP_DM` channel
    pub icon_hash:              String,
    /// creator of `GROUP_DM`, `PUBLIC_THREAD` or `PRIVATE_THREAD` channel
    pub owner:                  &'a user::User,
    // application_id
    /// for `GROUP_DM` channels: whether the channel is managed by an application via the `gdm.join` OAuth2 scope
    pub managed:                bool,
    /// for `GUILD_*` channels: parent `GUILD_CATEGORY` for a channel (each parent category can contain up to 50 channels)
    /// 
    /// for `*_THREADS` channels: text channel this thread was created
    pub parent:                 &'a channel::Channel<'a>,

    // last_pin_timestamp

    /// voice region id for `GUILD_VOICE` channel
    pub rtc_region:             String,
    /// the camera `VideoQualityMode` of `GUILD_VOICE` channel
    pub video_quality_mode:     ChannelVideoQualityMode,
    
    // message_count
    // member_count
    // thread_metadata
    // member
    // default_auto_archive_duration
    // permissions
    
    pub flags:                  Vec<ChannelFlag>,

    // total_message_sent
    // available tags
    // applied tags
    // default_reaction_emoji
    // default_thread_rate_limit
    pub default_sort_order:     ChannelSortOrder,
    pub default_forum_layout:   ChannelForumLayout,

    // pub(crate) client_ref:      &'a crate::Client<'a>,
}

impl<'a> Channel<'a> {

    pub fn send() {}
    pub fn modify() {}
    pub fn bulk_delete() {}
    pub fn close() {}

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
pub enum ChannelVideoQualityMode {
    AUTO = 1,
    FULL = 2
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ChannelFlag {
    PINNED                      = 1<<1,
    REQUIRE_TAG                 = 1<<4,
    HIDE_MEDIA_DOWNLOAD_OPTIONS = 1<<15
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ChannelSortOrder {
    LATEST_ACTIVITY,
    CREATION_DATE
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ChannelForumLayout {
    NOT_SET,
    LIST_VIEW,
    GALLERY_VIEW
}