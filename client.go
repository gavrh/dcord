package discord

import "fmt"

// # Client Struct
type Client struct {
	User		ClientUser
	Guilds		GuildManager
	Channels	ChannelManager
	Users		UserManager
	Members		any // MemberManager
	Messages	any // MessageManager
	Roles		any // RoleManager
	Reactions	any // ReactionManager
	Session		Session
}
// client login
func (c *Client) Login(token string) {
	println(token)
	// login
	// fetch data
	c.User = ClientUser{
		Id: "12345",
		Username: "ananya0807",
		Persona: "ananya",
		Bot: true,
	}
	// session
	// OperatingSystem	string
	// Application		any
	// Token			string
	// Intents			int
	// IntentsArray	[]Intent
	// Partials		any
	// EventCallbacks	map[string]any
	// whatever 
	// 
	fmt.Printf("%s is now online...\n", c.User.Persona)
}
// client events
func (c *Client) EventCallback(e Event, cb func(c *Client, d ...any)) {
	c.Session.Data.EventCallbacks[string(e)] = cb
}