mod shard;
mod voice;
mod ws;

use std::borrow::BorrowMut;

pub use shard::*;
pub use voice::*;
pub use ws::*;

use crate::utils::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayOpcode {
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

#[derive(Debug)]
pub struct GatewayIntents(Vec<GatewayIntent>);

impl GatewayIntents {

    pub fn new(intents: Vec<GatewayIntent>) -> Self {
        Self(intents)
    }

    pub fn bitwise(&self) -> u32 {
        let mut res: u32 = 0;
        for i in self.0.iter() {
            res = res + i.clone() as u32;
        }
        res
    }

}

/// [Gateway Intents] are bitwise values passed when connecting to the gateway when identifying
/// which correlate to a set of related events. If you do not specify an intent, you will not receive
/// any of the gateway events associated with that intent.
/// 
/// [Privileged Intents] require you to enable them in your app's settings. In the [Developer Portal], 
/// you can navigate to your app's settings then toggle the privileged intents on the Bots page under the 
/// "Privileged Gateway Intents" section. You should only toggle privileged intents that your bot requires to function.
/// 
/// **Current Privileged Intents:**
/// - [`GatewayIntent::GuildMembers`]
/// - [`GatewayIntent::GuildPresences`]
/// - [`GatewayIntent::MessageContent`]
/// 
/// **PS:** If your app qualifies for verification, you must first [verify your app] and request access to these intents
/// during the verification process. If your app is already verified and you need to request additional privileged
/// intents, you can [contact Discord support].
/// 
/// [Gateway Intents]: https://discord.com/developers/docs/topics/gateway#gateway-intents
/// [Privileged Intents]: https://discord.com/developers/docs/topics/gateway#privileged-intents
/// [Developer Portal]: https://discord.com/developers/applications
/// [verify your app]: https://support.discord.com/hc/en-us/articles/360040720412-Bot-Verification-and-Data-Allowlisting
/// [contact Discord support]: https://dis.gd/support
#[derive(Clone, Debug, Serialize)]
pub enum GatewayIntent {
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::GuildCreate`]
    /// - [`GatewayEvent::GuildUpdate`]
    /// - [`GatewayEvent::GuildDelete`]
    /// - [`GatewayEvent::GuildRoleCreate`]
    /// - [`GatewayEvent::GuildRoleUpdate`]
    /// - [`GatewayEvent::GuildRoleDelete`]
    /// - [`GatewayEvent::ChannelCreate`]
    /// - [`GatewayEvent::ChannelUpdate`]
    /// - [`GatewayEvent::ChannelDelete`]
    /// - [`GatewayEvent::ChannelPinsUpdate`]
    /// - [`GatewayEvent::ThreadCreate`]
    /// - [`GatewayEvent::ThreadUpdate`]
    /// - [`GatewayEvent::ThreadDelete`]
    /// - [`GatewayEvent::ThreadListSync`]
    /// - [`GatewayEvent::ThreadMemberUpdate`]
    /// - [`GatewayEvent::ThreadMembersUpdate`]
    /// - [`GatewayEvent::StageInstanceCreate`]
    /// - [`GatewayEvent::StageInstanceUpdate`]
    /// - [`GatewayEvent::StageInstanceDelete`]
    Guilds = 1<<0,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::GuildMemberAdd`]
    /// - [`GatewayEvent::GuildMemberUpdate`]
    /// - [`GatewayEvent::GuildMemberRemove`]
    /// - [`GatewayEvent::ThreadMembersUpdate`]
    GuildMembers = 1<<1,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::GuildAuditLogEntryCreate`]
    /// - [`GatewayEvent::GuildBanAdd`]
    /// - [`GatewayEvent::GuildBanRemove`]
    GuildModeration = 1<<2,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::GuildEmojisUpdate`]
    /// - [`GatewayEvent::GuildStickersUpdate`]
    GuildEmojisAndStickers = 1<<3,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::GuildIntegrationsUpdate`]
    /// - [`GatewayEvent::IntegrationCreate`]
    /// - [`GatewayEvent::IntegrationUpdate`]
    /// - [`GatewayEvent::IntegrationDelete`]
    GuildIntegrations = 1<<4,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::WebhooksUpdate`]
    GuildWebhooks = 1<<5,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::InviteCreate`]
    /// - [`GatewayEvent::InviteDelete`]
    GuildInvites = 1<<6,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::VoiceStateUpdate`]
    GuildVoiceStates = 1<<7,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::PresenceUpdate`]
    GuildPresences = 1<<8,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::MessageCreate`]
    /// - [`GatewayEvent::MessageUpdate`]
    /// - [`GatewayEvent::MessageDelete`]
    /// - [`GatewayEvent::MessageDeleteBulk`]
    GuildMessages = 1<<9,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::MessageReactionAdd`]
    /// - [`GatewayEvent::MessageReactionRemove`]
    /// - [`GatewayEvent::MessageReactionRemoveAll`]
    /// - [`GatewayEvent::MessageReactionRemoveEmoji`]
    GuildMessageReactions = 1<<10,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::TypingStart`]
    GuildMessageTyping = 1<<11,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::MessageCreate`]
    /// - [`GatewayEvent::MessageUpdate`]
    /// - [`GatewayEvent::MessageDelete`]
    /// - [`GatewayEvent::MessageDeleteBulk`]
    DirectMessages = 1<<12,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::MessageReactionAdd`]
    /// - [`GatewayEvent::MessageReactionRemove`]
    /// - [`GatewayEvent::MessageReactionRemoveAll`]
    /// - [`GatewayEvent::MessageReactionRemoveEmoji`]
    DirectMessageReactions= 1<<13,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::TypingStart`]
    DirectMessageTyping = 1<<14,
    /// Enables viewing contents of messages.
    MessageContent = 1<<15,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::GuildScheduledEventCreate`]
    /// - [`GatewayEvent::GuildScheduledEventUpdate`]
    /// - [`GatewayEvent::GuildScheduledEventDelete`]
    /// - [`GatewayEvent::GuildScheduledEventUserAdd`]
    /// - [`GatewayEvent::GuildScheduledEventUserRemove`]
    GuildScheduledEvents = 1<<16,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::AutoModerationRuleCreate`]
    /// - [`GatewayEvent::AutoModerationRuleUpdate`]
    /// - [`GatewayEvent::AutoModerationRuleDelete`]
    AutoModerationConfiguration = 1<<20,
    /// Enables the following gateway events:
    /// 
    /// [`GatewayEvent::AutoModerationActionExecution`]
    AutoModerationExecution = 1<<21,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::MessagePollVoteAdd`]
    /// - [`GatewayEvent::MessagePollVoteRemove`]
    GuildMessagePolls = 1<<24,
    /// Enables the following gateway events:
    /// 
    /// - [`GatewayEvent::MessagePollVoteAdd`]
    /// - [`GatewayEvent::MessagePollVoteRemove`]
    DirectMessagePolls = 1<<25,
    /// Enables all gateway events.
    All = 53608447
}

/// [Gateway Events] are received and encapsulated in an event payload, and are sent by Discord
/// to an app through a Gateway connection. Receive events correspond to events that happen in a
/// Discord server where the app is installed.
/// 
/// [Gateway Events]: https://discord.com/developers/docs/topics/gateway-events#receive-events
#[derive(Debug, Serialize)]
pub enum GatewayEvent {
    /// Application command permission was updated.
    ApplicationCommandPermissionsUpdate,
    /// Auto Moderation rule was create.
    AutoModerationRuleCreate,
    /// Auto Moderation rule was updated.
    AutoModerationRuleUpdate,
    /// Auto Moderation rule was deleted.
    AutoModerationRuleDelete,
    /// Auto Moderation rule was triggered and an action was executed.
    AutoModerationActionExecution,
    /// New guild channel created.
    ChannelCreate,
    /// Channel was updated.
    ChannelUpdate,
    /// Channel was deleted.
    ChannelDelete,
    /// Message was pinned or unpinned.
    ChannelPinsUpdate,
    /// Thread created, also sent when being added to a private thread.
    ThreadCreate,
    /// Thread was updated.
    ThreadUpdate,
    /// Thread was deleted.
    ThreadDelete,
    /// Sent when gaining access to a channel, contains all active threads in that channel.
    ThreadListSync,
    /// Thread member for the current user was updated.
    ThreadMemberUpdate,
    /// Some user(s) were added to or removed from a thread.
    ThreadMembersUpdate,
    /// Entitlement was created.
    EntitlementCreate,
    /// Entitlement was updated.
    EntitlementUpdate,
    /// Entitlement was deleted.
    EntitlementDelete,
    /// Lazy-load for unavailable guild, guild became available, or user joined a new guild.
    GuildCreate,
    /// Guild was updated.
    GuildUpdate,
    /// Guild became unavailable, or user left / was removed from a guild.
    GuildDelete,
    /// A guild audit log entry was created.
    GuildAuditLogEntryCreate,
    /// A user was banned from a guild.
    GuildBanAdd,
    /// A user was unbanned from a guild.
    GuildBanRemove,
    /// Guild emojis were updated.
    GuildEmojisUpdate,
    /// Guild stickers were updated.
    GuildStickersUpdate,
    /// Guild integration was updated.
    GuildIntegrationsUpdate,
    /// New user joined a guild.
    GuildMemberAdd,
    /// Guild member was updated.
    GuildMemberUpdate,
    /// User was removed from a guild.
    GuildMemberRemove,
    /// Response to *Request Guild Members*.
    GuildMembersChunck,
    /// Guild role was created.
    GuildRoleCreate,
    /// Guild role was updated.
    GuildRoleUpdate,
    /// Guild role was deleted.
    GuildRoleDelete,
    /// Guild scheduled event was created.
    GuildScheduledEventCreate,
    /// Guild scheduled event was updated.
    GuildScheduledEventUpdate,
    /// Guild scheduled event was deleted.
    GuildScheduledEventDelete,
    /// A user subscribed to a guild scheduled event.
    GuildScheduledEventUserAdd,
    /// A user unsubscribed to a guild scheduled event.
    GuildScheduledEventUserRemove,
    /// Guild integration was created.
    IntegrationCreate,
    /// Guild integration was updated.
    IntegrationUpdate,
    /// Guild integration was deleted.
    IntegrationDelete,
    /// A user used an interaction, such as an *Application Command*.
    InteractionCreate,
    /// Invite to channel was created.
    InviteCreate,
    /// Invite to channel was deleted.
    InviteDelete,
    /// Message was created.
    MessageCreate,
    /// Message was edited.
    MessageUpdate,
    /// Message was deleted.
    MessageDelete,
    /// Multiple messages were deleted at once.
    MessageDeleteBulk,
    /// A user reacted to a message.
    MessageReactionAdd,
    /// A user removed a reaction from a message.
    MessageReactionRemove,
    /// All reactions were explicitly removed from a message.
    MessageReactionRemoveAll,
    /// All reactions for a given emoji were explicitly removed from a message.
    MessageReactionRemoveEmoji,
    /// A user's presence was updated.
    PresenceUpdate,
    /// Stage instance was created.
    StageInstanceCreate,
    /// Stage instance was updated.
    StageInstanceUpdate,
    /// Stage instance was deleted.
    StageInstanceDelete,
    /// A user started typing in a channel.
    TypingStart,
    /// Properties about the user changed.
    UserUpdate,
    /// Someone joined, left, or moved a voice channel.
    VoiceStateUpdate,
    /// Guild's voice server was updated.
    VoiceServerUpdate,
    /// Guild channel webhook was created, updated, or deleted.
    WebhooksUpdate,
    /// A user voted on a poll.
    MessagePollVoteAdd,
    /// A user removed a vote on a poll.
    MessagePollVoteRemove
}

pub mod gateway_close_codes {
    /// Unknown error; try reconnecting?
    /// 
    /// Can reconnect.
    pub const UNKNOWN_ERROR: u16 = 4000;
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