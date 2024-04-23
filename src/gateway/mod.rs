mod ws;

pub use ws::*;

use crate::utils::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    Dispatch            = 0,
    Heartbeat           = 1,
    Identify            = 2,
    PresenceUpdate      = 3,
    VoiceStateUpdate    = 4,
    Resume              = 6,
    Reconnect           = 7,
    RequestGuildMembers = 8,
    InvalidSession      = 9,
    Hello               = 10,
    HeartbeatAck        = 11
}

#[derive(Debug, Serialize)]
pub enum GatewayIntent {
    Guilds                      = 1<<0,
    GuildMembers                = 1<<1,
    GuildModeration             = 1<<2,
    GuildEmojisAndStickers      = 1<<3,
    GuildIntegrations           = 1<<4,
    GuildWebhooks               = 1<<5,
    GuildInvites                = 1<<6,
    GuildVoiceStates            = 1<<7,
    GuildPresences              = 1<<8,
    GuildMessages               = 1<<9,
    GuildMessageReactions       = 1<<10,
    GuildMessageTyping          = 1<<11,
    DirectMessages              = 1<<12,
    DirectMessageReactions      = 1<<13,
    DirectMessageTyping         = 1<<14,
    MessageContent              = 1<<15,
    GuildScheduledEvents        = 1<<16,
    AutoModerationConfiguration = 1<<20,
    AutoModerationExecution     = 1<<21,
    GuildMessagePolls           = 1<<24,
    DirectMessagePolls          = 1<<25,
    All                         = 53608447
}

pub mod close_codes {
    /// Unknown error; try reconnecting?
    /// 
    /// Can reconnect.
    pub const UNKNOWN_ERROR: u16    = 4000;
    /// Invalid Gateway OP Code.
    /// 
    /// Can resume.
    pub const UNKNOWN_OPCODE: u16 = 4001;
    /// An invalid payload was sent.
    /// 
    /// Can resume.
    pub const DECODE_ERROR: u16 = 4002;
    /// A payload was sent prior to identifying.
    /// 
    /// Cannot reconnect.
    pub const NOT_AUTHENTICATED: u16 = 4003;
    /// The account token sent with the identify payload was incorrect.
    ///
    /// Cannot reconnect.
    pub const AUTHENTICATION_FAILED: u16 = 4004;
    /// More than one identify payload was sent.
    ///
    /// Can reconnect.
    pub const ALREADY_AUTHENTICATED: u16 = 4005;
    /// The sequence sent when resuming the session was invalid.
    ///
    /// Can reconnect.
    pub const INVALID_SEQUENCE: u16 = 4007;
    /// Payloads were being sent too quickly.
    ///
    /// Can resume.
    pub const RATE_LIMITED: u16 = 4008;
    /// A session timed out.
    ///
    /// Can reconnect.
    pub const SESSION_TIMEOUT: u16 = 4009;
    /// An invalid shard when identifying was sent.
    ///
    /// Cannot reconnect.
    pub const INVALID_SHARD: u16 = 4010;
    /// The session would have handled too many guilds.
    ///
    /// Cannot reconnect.
    pub const SHARDING_REQUIRED: u16 = 4011;
    /// Undocumented gateway intents have been provided.
    pub const INVALID_GATEWAY_INTENTS: u16 = 4013;
    /// Disallowed gateway intents have been provided.
    pub const DISALLOWED_GATEWAY_INTENTS: u16 = 4014;
}