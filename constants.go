package discord

// opcode type and constants (private)
type opcode int
const (
  opcode_DISPATCH               opcode = 0
  opcode_HEARTBEAT              opcode = 1
  opcode_IDENTIFY               opcode = 2
  opcode_PRESENCE_UPDATE        opcode = 3
  opcode_VOICE_STATE_UPDATE     opcode = 4
  opcode_RESUME                 opcode = 6
  opcode_RECONNECT              opcode = 7
  opcode_REQUEST_GUILD_MEMBERS  opcode = 8
  opcode_INVALID_SESSION        opcode = 9
  opcode_HELLO                  opcode = 10
  opcode_HEARTBEAT_ACK          opcode = 11
)

// intent type and constants
type Intent int
const (
  Intent_ALL                            Intent = 3276799
  Intent_GUILDS                         Intent = 1<<0  
  Intent_GUILD_MEMBERS                  Intent = 1<<1 
  Intent_GUILD_MODERATION               Intent = 1<<2
  Intent_GUILD_EMOJIS_AND_STICKERS      Intent = 1<<3
  Intent_GUILD_INTEGRATIONS             Intent = 1<<4
  Intent_GUILD_WEBHOOKS                 Intent = 1<<5
  Intent_GUILD_INVITES                  Intent = 1<<6
  Intent_GUILD_VOICE_STATES             Intent = 1<<7
  Intent_GUILD_PRESENCES                Intent = 1<<8
  Intent_GUILD_MESSAGES                 Intent = 1<<9
  Intent_GUILD_MESSAGE_REACTIONS        Intent = 1<<10
  Intent_GUILD_MESSAGE_TYPING           Intent = 1<<11
  Intent_DIRECT_MESSAGES                Intent = 1<<12
  Intent_DIRECT_MESSAGE_REACTIONS       Intent = 1<<13
  Intent_DIRECT_MESSAGE_TYPING          Intent = 1<<14
  Intent_MESSAGE_CONTENT                Intent = 1<<15
  Intent_GUILD_SCHEDULED_EVENTS         Intent = 1<<16
  Intent_GUILD_MODERATION_CONFIGURATION Intent = 1<<20
  Intent_GUILD_MODERATION_EXECUTION     Intent = 1<<21
)
// event type and constants
type Event string
const (
  Event_GUILD_CREATE                      Event = "GUILD_CREATE"
  Event_GUILD_UPDATE                      Event = "GUILD_UPDATE"
  Event_GUILD_DELETE                      Event = "GUILD_DELETE"
  Event_GUILD_ROLE_CREATE                 Event = "GUILD_ROLE_CREATE"
  Event_GUILD_ROLE_UPDATE                 Event = "GUILD_ROLE_UPDATE"
  Event_GUILD_ROLE_DELETE                 Event = "GUILD_ROLE_DELETE"
  Event_CHANNEL_CREATE                    Event = "CHANNEL_CREATE"
  Event_CHANNEL_UPDATE                    Event = "CHANNEL_UPDATE"
  Event_CHANNEL_DELETE                    Event = "CHANNEL_DELETE"
  Event_CHANNEL_PINS_UPDATE               Event = "CHANNEL_PINS_UPDATE"
  Event_THREAD_CREATE                     Event = "THREAD_CREATE"
  Event_THREAD_UPDATE                     Event = "THREAD_UPDATE"
  Event_THREAD_DELETE                     Event = "THREAD_DELETE"
  Event_THREAD_MEMBER_SYNC                Event = "THREAD_MEMBER_SYNC"
  Event_THREAD_MEMBER_UPDATE              Event = "THREAD_MEMBER_UPDATE"
  Event_THREAD_MEMBERS_UPDATE             Event = "THREAD_MEMBERS_UPDATE"
  Event_STAGE_INSTANCE_CREATE             Event = "STAGE_INSTANCE_CREATE"
  Event_STAGE_INSTANCE_UPDATE             Event = "STAGE_INSTANCE_UPDATE"
  Event_STAGE_INSTANCE_DELETE             Event = "STAGE_INSTANCE_DELETE"
  Event_GUILD_MEMBER_ADD                  Event = "GUILD_MEMBER_ADD"
  Event_GUILD_MEMBER_UPDATE               Event = "GUILD_MEMBER_UPDATE"
  Event_GUILD_MEMBER_REMOVE               Event = "GUILD_MEMBER_REMOVE"
  Event_GUILD_AUDIT_LOG_ENTRY_CREATE      Event = "GUILD_AUDIT_LOG_ENTRY_CREATE"
  Event_INTEGRATION_CREATE                Event = "INTEGRATION_CREATE"
  Event_INTEGRATION_UPDATE                Event = "INTEGRATION_UPDATE"
  Event_INTEGRATION_DELETE                Event = "INTEGRATION_DELETE"
  Event_WEBHOOKS_UPDATE                   Event = "WEBHOOKS_UPDATE"
  Event_INVITE_CREATE                     Event = "INVITE_CREATE"
  Event_INVITE_DELETE                     Event = "INVITE_DELETE"
  Event_VOICE_STATE_UPDATE                Event = "VOICE_STATE_UPDATE"
  Event_PRESENCE_UPDATE                   Event = "PRESENCE_UPDATE"
  Event_MESSAGE_CREATE                    Event = "MESSAGE_CREATE"
  Event_MESSAGE_UPDATE                    Event = "MESSAGE_UPDATE"
  Event_MESSAGE_DELETE                    Event = "MESSAGE_DELETE"
  Event_MESSAGE_DELETE_BULK               Event = "MESSAGE_DELETE_BULK"
  Event_MESSAGE_REACTION_ADD              Event = "MESSAGE_REACTION_ADD"
  Event_MESSAGE_REACTION_REMOVE           Event = "MESSAGE_REACTION_REMOVE"
  Event_MESSAGE_REACTION_REMOVE_ALL       Event = "MESSAGE_REACTION_REMOVE_ALL"
  Event_MESSAGE_REACTION_REMOVE_EMOJI     Event = "MESSAGE_REACTION_REMOVE_EMOJI"
  Event_TYPING_START                      Event = "TYPING_START"
  Event_GUILD_SCHEDULED_EVENT_CREATE      Event = "GUILD_SCHEDULED_EVENT_CREATE"
  Event_GUILD_SCHEDULED_EVENT_UPDATE      Event = "GUILD_SCHEDULED_EVENT_UPDATE"
  Event_GUILD_SCHEDULED_EVENT_DELETE      Event = "GUILD_SCHEDULED_EVENT_DELETE"
  Event_GUILD_SCHEDULED_EVENT_USER_ADD    Event = "GUILD_SCHEDULED_EVENT_USER_ADD"
  Event_GUILD_SCHEDULED_EVENT_USER_REMOVE Event = "GUILD_SCHEDULED_EVENT_USER_REMOVE"
  Event_AUTO_MODERATION_RULE_CREATE       Event = "AUTO_MODERATION_RULE_CREATE"
  Event_AUTO_MODERATION_RULE_UPDATE       Event = "AUTO_MODERATION_RULE_UPDATE"
  Event_AUTO_MODERATION_RULE_DELETE       Event = "AUTO_MODERATION_RULE_DELETE"
  Event_AUTO_MODERATION_ACTION_EXECUTION  Event = "AUTO_MODERATION_ACTION_EXECUTION"
)



// error code type and constants
type Error struct {
  Code        int
  Description string
  reconnect   bool
}
