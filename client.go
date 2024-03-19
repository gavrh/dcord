package discord

// # Client Struct
type Client struct {
	User		ClientUser
	Guilds		guildManager
	Channels	channelManager
	Users		userManager
	Members		any // MemberManager
	Messages	any // MessageManager
	Roles		any // RoleManager
	Reactions	any // ReactionManager
	Session		Session

	// event callbacks
	onReady			func(*Client)
	onGuildCreate	func(*Client, *Guild)
	onGuildUpdate	func(*Client, *Guild, Guild)
	onGuildDelete	func(*Client, Guild)
	onMessageCreate func(*Client, *Message)
	onMessageUpdate func(*Client, *Message, Message)
	onMessageDelete func(*Client, Message)
}
// client login
func (c *Client) Login(token string) {
	println("token: "+token)
	c.Session.Data.Token = token
	done := make(chan bool)
	go c.dialGateway(done, token)
	
	// keep alive
	for {
		select {
		case <-done:
			return
		}
	}
}

// event callbacks
type DebugMessage string
func (c *Client) Debug(callback func(client *Client, debug_message *DebugMessage)) {  }
func (c *Client) Ready(callback func(client *Client)) { c.onReady = callback }
// GUILDS //
func (c *Client) GuildCreate(callback func(client *Client, guild *Guild)) { c.onGuildCreate = callback }
func (c *Client) GuildUpdate(callback func(client *Client, guild *Guild, old_guild Guild)) { c.onGuildUpdate = callback }
func (c *Client) GuildDelete(callback func(client *Client, guild Guild)) { c.onGuildDelete = callback }
// GuildRoleCreate
// GuildRoleUpdate
// GuildRoleDelete
// ChannelCreate
// ChannelUpdate
// ChannelDelete
// ChannelPinsUpdate
// ThreadListSync
// ThreadMemberUpdate
// ThreadMembersUpdate
// StageInstanceCreate
// StageInstanceUpdate
// StageInstanceDelete
// GUILD MEMBERS //
// GuildMemberAdd
// GuildMemberUpdate
// GuildMemberDelete
// GUILD BANS //
// GuildBanAdd
// GuildBanRemove
// GUILD EMOJIS AND STICKERS //
// GuildEmojisUpdate
// GuildStickersUpdate
// GUILD INTEGRATIONS //
// GuildIntegrationsUpdate
// IntegrationCreate
// IntegrationUpdate
// IntegrationDelete
// GUILD WEBHOOKS //
// WebhooksUpdate
// GUILD INVITES
// InviteCreate
// InviteDelete
// GUILD VOICE STATES //
// VoiceStateUpdate
// GUILD PRESENCES //
// PresenceUpdate
// (GUILD / DIRECT) MESSAGES //
func (c *Client) MessageCreate(callback func(client *Client, message *Message)) { c.onMessageCreate = callback }
func (c *Client) MessageUpdate(callback func(client *Client, message *Message, old_message Message)) { c.onMessageUpdate = callback }
func (c *Client) MessageDelete(callback func(client *Client, message Message)) { c.onMessageDelete = callback }
// MessageBulkDelete
// GUILD MESSAGE TYPING //
// TypingStart
// GUILD SCHEDULED EVENTS //
// GuildScheduledEventCreate
// GuildScheduledEventUpdate
// GuildScheduledEventDelete
// GuildScheduledEventUserAdd
// GuildScheduledEventUserRemove
// AUTO MOD CONFIGURATION //
// AutoModRuleCreate
// AutoModRuleUpdate
// AutoModRuleDelete
// AUTO MOD EXECUTION //
// AutoModActionExecution