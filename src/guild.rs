use crate::manager;
use crate::channel;
use crate::member;

#[derive(Debug)]
pub struct Guild<'a> {
    pub id:                                     String,
    // name

    // joined_at
    // large
    // unavailable
    // member_count
    // voice_states
    pub members:                                manager::Manager<'a, member::Member<'a>>,
    pub channels:                               manager::Manager<'a, channel::Channel<'a>>, 
    // threads
    // presences
    // stage_instances
    // guild_scheduled_events

    // icon
    // icon_hash
    // splash
    // discovery_splash
    // owner
    // afk_channel *
    // afk_timeout
    // widget_enabled
    // widget_channel *
    pub verification_level:                     GuildVerificationLevel,
    pub default_message_notifications_level:    GuildDefaultMessageNotificationsLevel,
    pub explicit_content_filter_level:          GuildExplicitContentFilterLevel,
    // roles (manager)
    // emojis (manager)
    pub features:                               Vec<GuildFeature>,
    pub mfa_level:                              GuildMfaLevel,
    // application
    // system_channel *
    // system_channel_flags (might be optional)
    // rules_channel *
    // max_presences
    // vanity_url
    // description
    // banner
    pub premium_tier:                           GuildPremiumTier,
    // premium_subscription_count
    pub preferred_locale:                       GuildLocale,
    // public_updates_channel *
    // max_video_channel_users
    // max_stage_video_channel_users
    // approximate_member_count
    // approximate_presence_count
    // welcome_screen
    pub nsfw_level:                             GuildNsfwLevel
    // stickers (manager)
    // premium_progress_bar_enabled
    // safety_alerts_channel *
}


#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildVerificationLevel {
    NONE,
    LOW,
    MEDIUM,
    HIGH,
    VERY_HIGH
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildDefaultMessageNotificationsLevel {
    ALL_MESSAGES,
    ONLY_MENTIONS
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildExplicitContentFilterLevel {
    DISABLED,
    MEMBERS_WITHOUT_ROLES,
    ALL_MEMBERS
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildFeature {
    ANIMATED_BANNER,
    ANIMATED_ICON,
    APPLICATION_COMMAND_PERMISSIONS_V2,
    AUTO_MODERATION,
    BANNER,
    COMMUNITY,
    CREATOR_MONETIZABLE_PROVISIONAL,
    CREATOR_STORE_PAGE,
    DEVELOPER_SUPPORT_SERVER,
    DISCOVERABLE,
    FEATURABLE,
    INVITES_DISABLED,
    INVITE_SPLASH,
    MEMBER_VERIFICATION_GATE_ENABLED,
    MORE_STICKERS,
    NEWS,
    PARTNERED,
    RAID_ALERTS_DISABLED,
    ROLE_ICONS,
    ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE,
    ROLE_SUBSCRIPTIONS_ENABLED,
    TICKETED_EVENTS_ENABLED,
    VANITY_URL,
    VERIFIED,
    VIP_REGIONS,
    WELCOME_SCREEN_ENABLED
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildMfaLevel {
    NONE,
    ELEVATED
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildPremiumTier {
    NONE,
    TIER_1,
    TIER_2,
    TIER_3
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildLocale {
    INDONESIAN,
    DANISH,
    GERMAN,
    ENGLISH_UK,
    ENGLISH_US,
    SPANISH,
    SPANISH_LATAM,
    FRENCH,
    CROATIAN,
    ITALIAN,
    LITHUANIAN,
    HUNGARIAN,
    DUTCH,
    NORWEGIAN,
    POLISH,
    PORTUGUESE,
    ROMANIAN,
    FINNISH,
    SWEDISH,
    VIETNAMESE,
    TURKISH,
    CZECH,
    GREEK,
    BULGARIAN,
    RUSSIAN,
    UKRAINIAN,
    HINDI,
    THAI,
    CHINESE_CN,
    JAPANESE,
    CHINESE_TW,
    KOREAN
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GuildNsfwLevel {
    DEFAULT,
    EXPLICIT,
    SAFE,
    AGE_RESTRICTED
}