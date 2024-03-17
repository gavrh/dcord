package discord

// event constants
type Event string
const (
	Event_HELLO                                  Event = "HELLO"
	Event_READY                                  Event = "READY"
	Event_RESUMED                                Event = "RESUMED"
	Event_RECONNECT                              Event = "RECONNECT"
	Event_INVALID_SESSION                        Event = "INVALID_SESSION"
	Event_APPLICATION_COMMAND_PERMISSIONS_UPDATE Event = "APPLICATION_COMMAND_PERMISSIONS_UPDATE"
	Event_AUTO_MODERATION_RULE_CREATE            Event = "AUTO_MODERATION_RULE_CREATE"
	Event_AUTO_MODERATION_RULE_UPDATE            Event = "AUTO_MODERATION_RULE_UPDATE"
	Event_AUTO_MODERATION_RULE_DELETE            Event = "AUTO_MODERATION_RULE_DELETE"
	Event_AUTO_MODERATION_ACTION_EXECUTION       Event = "AUTO_MODERATION_ACTION_EXECUTION"
	Event_CHANNEL_CREATE                         Event = "CHANNEL_CREATE"
	Event_CHANNEL_UPDATE                         Event = "CHANNEL_UPDATE"
	Event_CHANNEL_DELETE                         Event = "CHANNEL_DELETE"
	Event_CHANNEL_PINS_UPDATE                    Event = "CHANNEL_PINS_UPDATE"
	Event_THREAD_CREATE                          Event = "THREAD_CREATE"
	Event_THREAD_UPDATE                          Event = "THREAD_UPDATE"
	Event_THREAD_DELETE                          Event = "THREAD_DELETE"
	Event_THREAD_LIST_SYNC                       Event = "THREAD_LIST_SYNC"
	Event_THREAD_MEMBER_UPDATE                   Event = "THREAD_MEMBER_UPDATE"
	Event_THREAD_MEMBERS_UPDATE                  Event = "THREAD_MEMBERS_UPDATE"
	Event_ENTITLEMENT_CREATE                     Event = "ENTITLEMENT_CREATE"
	Event_ENTITLEMENT_UPDATE                     Event = "ENTITLEMENT_UPDATE"
	Event_ENTITLEMENT_DELETE                     Event = "ENTITLEMENT_DELETE"
	Event_GUILD_CREATE                           Event = "GUILD_CREATE"
	Event_GUILD_UPDATE                           Event = "GUILD_UPDATE"
	Event_GUILD_DELETE                           Event = "GUILD_DELETE"
	Event_GUILD_AUDIT_LOG_ENTRY_CREATE           Event = "GUILD_AUDIT_LOG_ENTRY_CREATE"
	Event_GUILD_BAN_ADD                          Event = "GUILD_BAN_ADD"
	Event_GUILD_BAN_REMOVE                       Event = "GUILD_BAN_REMOVE"
	Event_GUILD_EMOJIS_UPDATE                    Event = "GUILD_EMOJIS_UPDATE"
	Event_GUILD_STICKERS_UPDATE                  Event = "GUILD_STICKERS_UPDATE"
	Event_GUILD_INTEGRATIONS_UPDATE              Event = "GUILD_INTEGRATIONS_UPDATE"
	Event_GUILD_MEMBER_ADD                       Event = "GUILD_MEMBER_ADD"
	Event_GUILD_MEMBER_REMOVE                    Event = "GUILD_MEMBER_REMOVE"
	Event_GUILD_MEMBER_UPDATE                    Event = "GUILD_MEMBER_UPDATE"
	Event_GUILD_MEMBER_CHUNK                     Event = "GUILD_MEMBER_CHUNK"
	Event_GUILD_ROLE_CREATE                      Event = "GUILD_ROLE_CREATE"
	Event_GUILD_ROLE_UPDATE                      Event = "GUILD_ROLE_UPDATE"
	Event_GUILD_ROLE_DELETE                      Event = "GUILD_ROLE_DELETE"
	Event_GUILD_SCHEDULED_EVENT_CREATE           Event = "GUILD_SCHEDULED_EVENT_CREATE"
	Event_GUILD_SCHEDULED_EVENT_UPDATE           Event = "GUILD_SCHEDULED_EVENT_UPDATE"
	Event_GUILD_SCHEDULED_EVENT_DELETE           Event = "GUILD_SCHEDULED_EVENT_DELETE"
	Event_GUILD_SCHEDULED_EVENT_USER_ADD         Event = "GUILD_SCHEDULED_EVENT_USER_ADD"
	Event_GUILD_SCHEDULED_EVENT_USER_REMOVE      Event = "GUILD_SCHEDULED_EVENT_USER_REMOVE"
	Event_INTEGRATION_CREATE                     Event = "INTEGRATION_CREATE"
	Event_INTEGRATION_UPDATE                     Event = "INTEGRATION_UPDATE"
	Event_INTEGRATION_DELETE                     Event = "INTEGRATION_DELETE"
	Event_INVITE_CREATE                          Event = "INVITE_CREATE"
	Event_INVITE_DELETE                          Event = "INVITE_DELETE"
	Event_MESSAGE_CREATE                         Event = "MESSAGE_CREATE"
	Event_MESSAGE_UPDATE                         Event = "MESSAGE_UPDATE"
	Event_MESSAGE_DELETE                         Event = "MESSAGE_DELETE"
	Event_MESSAGE_DELETE_BULK                    Event = "MESSAGE_DELETE_BULK"
	Event_MESSAGE_REACTION_ADD                   Event = "MESSAGE_REACTION_ADD"
	Event_MESSAGE_REACTION_REMOVE                Event = "MESSAGE_REACTION_REMOVE"
	Event_MESSAGE_REACTION_REMOVE_ALL            Event = "MESSAGE_REACTION_REMOVE_ALL"
	Event_MESSAGE_REACTION_REMOVE_EMOJI          Event = "MESSAGE_REACTION_REMOVE_EMOJI"
	Event_PRESENCE_UPDATE                        Event = "PRESENCE_UPDATE"
	Event_STAGE_INSTANCE_CREATE                  Event = "STAGE_INSTANCE_CREATE"
	Event_STAGE_INSTANCE_UPDATE                  Event = "STAGE_INSTANCE_UPDATE"
	Event_STAGE_INSTANCE_DELETE                  Event = "STAGE_INSTANCE_DELETE"
	Event_TYPING_START                           Event = "TYPING_START"
	Event_USER_UPDATE                            Event = "USER_UPDATE"
	Event_VOICE_STATE_UPDATE                     Event = "VOICE_STATE_UPDATE"
	Event_VOICE_SERVER_UPDATE                    Event = "VOICE_SERVER_UPDATE"
	Event_WEBHOOKS_UPDATE                        Event = "WEBHOOKS_UPDATE"
)

// intent constants
type Intent int
const (
	Intent_ALL                           Intent = 3276799
	Intent_GUILDS                        Intent = 1 << 0
	Intent_GUILD_MEMBERS                 Intent = 1 << 1
	Intent_GUILD_MODERATION              Intent = 1 << 2
	Intent_GUILD_EMOJIS_AND_STICKERS     Intent = 1 << 3
	Intent_INTEGRATIONS                  Intent = 1 << 4
	Intent_WEBHOOKS                      Intent = 1 << 5
	Intent_INVITES                       Intent = 1 << 6
	Intent_GUILD_VOICE_STATES            Intent = 1 << 7
	Intent_GUILD_PRESENCES               Intent = 1 << 8
	Intent_GUILD_MESSAGES                Intent = 1 << 9
	Intent_GUILD_MESSAGE_REACTIONS       Intent = 1 << 10
	Intent_GUILD_MESSAGE_TYPING          Intent = 1 << 11
	Intent_DIRECT_MESSAGES               Intent = 1 << 12
	Intent_DIRECT_MESSAGE_REACTIONS      Intent = 1 << 13
	Intent_DIRECT_MESSAGE_TYPING         Intent = 1 << 14
	Intent_MESSAGE_CONTENT               Intent = 1 << 15
	Intent_GUILD_SCHEDULED_EVENTS        Intent = 1 << 16
	Intent_AUTO_MODERATION_CONFIGURATION Intent = 1 << 20
	Intent_AUTO_MODERATION_EXECUTION     Intent = 1 << 21
)

// user flag constants
type UserFlag int
const (
	UserFlag_STAFF                   UserFlag = 1 << 0
	UserFlag_PARTNER                 UserFlag = 1 << 1
	UserFlag_HYPESQUAD               UserFlag = 1 << 2
	UserFlag_BUG_HUNTER_LEVEL_1      UserFlag = 1 << 3
	UserFlag_HYPSQUAD_ONLINE_HOUSE_1 UserFlag = 1 << 6
	UserFlag_HYPSQUAD_ONLINE_HOUSE_2 UserFlag = 1 << 7
	UserFlag_HYPSQUAD_ONLINE_HOUSE_3 UserFlag = 1 << 8
	UserFlag_PREMIUM_EARLY_SUPPORTER UserFlag = 1 << 9
	UserFlag_TEAM_PSEUDO_USER        UserFlag = 1 << 10
	UserFlag_BUG_HUNTER_LEVEL_2      UserFlag = 1 << 14
	UserFlag_VERIFIED_BOT            UserFlag = 1 << 16
	UserFlag_VERIFIED_DEVELOPER      UserFlag = 1 << 17
	UserFlag_CERTIFIED_MODERATOR     UserFlag = 1 << 18
	UserFlag_BOT_HTTP_INTERACTIONS   UserFlag = 1 << 19
	UserFlag_ACTIVE_DEVELOPER        UserFlag = 1 << 22
)

// user premium constants
type UserPremium int
const (
	UserPremium_NONE          UserPremium = 0
	UserPremium_NITRO_CLASSIC UserPremium = 1
	UserPremium_NITRO         UserPremium = 2
	UserPremium_NITRO_BASIC   UserPremium = 3
)
