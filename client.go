package discord

import "runtime"

// client struct
type Client struct {
  User      *ClientUser
  Users     userManager
  Channels  channelManager
  Guilds    guildManager

  // callbacks
  cbDebug   func(*Client, string)
  cbReady   func (*Client)

  // private
  session   session
}
// New Client
func NewClient(intents []Intent, partials int, api_version int, shards int) Client {
  return Client{
    Users:        userManager{users: make(map[string]*User)},
    Channels:     channelManager{channels: make(map[string]*Channel)},
    Guilds:       guildManager{guilds: make(map[string]*Guild)},

    session: session{
      ApiVersion: api_version,
      Data:       sessionData{
        OS:       runtime.GOOS,
        Intents:  int(Intent_ALL),
        Partials: partials,
      },
    },
  }
}

// callback intializers
func (c *Client) OnDebug(callback func(*Client, string)) { c.cbDebug = callback }
func (c *Client) OnReady(callback func(*Client)) { c.cbReady = callback }
