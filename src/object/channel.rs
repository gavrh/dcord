pub enum ChannelKind {
    /// A text channel within a guild.
    GuildText,
    /// A direct message channel between users.
    DM,
    /// A voice channel within a guild.
    GuildVoice,
    /// A direct message channel between multiple users.
    GroupDM,
    /// An organized category that contains up to 50 channels.
    GuildCategory,
    /// A channel that users can follow and crosspost into their own [`Guild`] (formerly news channels).
    GuildAnnouncement,
    /// A temporary sub-channel within a [`GuildAnnouncement`](ChannelKind::GuildAnnouncement) channel.
    AnnouncementThread = 10,
    /// A temporary sub-channel within a [`GuildText`](ChannelKind::GuildText) or
    /// [`GuildForum`](ChannelKind::GuildForum) channel.
    PublicThread,
    /// A temporary sub-channel within a [`GuildText`](ChannelKind::GuildText) channel that is
    /// only viewable by those invited and those with the [`ManageThreads`](MemberPermission::ManageThreads) permission
    PrivateThread,
    /// A voice channel for hosting events with an audience.
    GuildStageVoice,
    /// The channel in a hub containing the listed servers.
    GuildDirectory,
    /// Channel that can only contain threads.
    /// 
    /// Similar to [`GuildMedia`](ChannelKind::GuildMedia) channels.
    GuildForum,
    /// Channel that can only contain threads.
    /// 
    /// Similar to [`GuildForum`](ChannelKind::GuildForum) channels.
    GuildMedia
}

pub enum VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    Auto = 1,
    /// 720p quality.
    Full
}

pub enum ChannelFlag {
    /// This thread is pinned to the top of its parent [`GuildForum`](ChannelKind::GuildForum)
    /// or [`GuildMedia`](ChannelKind::GuildMedia) channel.
    Pinned = 1<<1,
    /// Wether a tag is required to be specified when creating a thread in a [`GuildForum`](ChannelKind::GuildForum)
    /// or a [`GuildMedia`](ChannelKind::GuildMedia) channel.
    RequireTag = 1<<4,
    /// When set, hides the embedded media download options. 
    /// 
    /// Available only for [`GuildMedia`](ChannelKind::GuildMedia) channels.
    HideMediaDownloadOptions = 1<<15
}

pub struct ForumChannelTag {
    pub id: String,
    pub name: String,
    pub moderated: bool,
    pub eomji: Option<String>, // replace later
}

pub enum ChannelSortOrder {
    /// Sort forum posts by activity.
    LatestActivity,
    /// Sort forum posts by creation time (from most recent to oldest).
    CreationDate
}

pub enum ForumChannelLayout {
    /// No default has been set for [`GuildForum`](ChannelKind::GuildForum) channel.
    NotSet,
    /// Display posts as a list.
    ListView,
    /// Display posts as a collection of tiles.
    GalleryView
}

pub struct Channel {
    /// The ID of this channel.
    pub id: String,
    /// The kind of channel.
    pub kind: ChannelKind,
    /// A reference to the guild this channel is in.
    pub guild: Option<String>, // replace later
    pub position: Option<u32>,
    pub permission_overwrites: Vec<u32>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message: Option<String>, // replace later
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<String>>, // replace later
    pub icon: Option<String>,
    pub owner: Option<String>, // replace later
    pub application: Option<String>, // replace later
    pub managed: Option<bool>,
    pub parent: Option<String>,
    pub last_pin_timestamp: Option<String>, // replace later
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub message_count: Option<u64>,
    pub member_count: Option<u64>,
    pub thread_metadata: Option<String>, // replace later
    pub member: Option<String>, // replace later
    pub default_auto_archive_duration: Option<u64>,
    pub permissions: Vec<super::MemberPermission>, // replace later
    pub flags: Vec<ChannelFlag>,
    pub total_message_sent: Option<u64>,
    pub available_tags: Option<Vec<ForumChannelTag>>,
    pub applied_tags: Option<Vec<ForumChannelTag>>,
    pub default_reaction_emoji: Option<String>, // replace later
    pub default_thread_rate_limit_per_user: Option<u64>,
    pub default_sort_order: Option<ChannelSortOrder>,
    pub default_forum_layout: Option<ForumChannelLayout>
}
