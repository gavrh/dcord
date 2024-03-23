package discord

import (
	"os"
  "os/signal"
	"runtime"
)

// client struct
type Client struct {
  User            *clientUser
  Users           userManager
  Channels        channelManager
  Guilds          guildManager

  // callbacks
  cbDebug         func(*Client, string)
  cbReady         func(*Client)
  cbGuildCreate   func(*Client, *Guild)
  cbGuildUpdate   func(*Client, *Guild, Guild)
  cbGuildDelete   func(*Client, Guild)
  cbChannelCreate func(*Client, *Channel)
  cbChannelUpdate func(*Client, *Channel, Channel)
  cbChannelDelete func(*Client, Channel)
  cbMessageCreate func(*Client, *Message)
  cbMessageUpdate func(*Client, *Message, Message)
  cbMessageDelete func(*Client, Message)
  
  // private
  session         session
}

// New Client
func NewClient(intents []Intent, partials int, api_version int, shards int, debug bool) Client {
  return Client{
    Users:        userManager{users: make(map[string]*User)},
    Channels:     channelManager{channels: make(map[string]*Channel)},
    Guilds:       guildManager{guilds: make(map[string]*Guild)},

    session: session{
      Data:         sessionData{
        OS:         runtime.GOOS,
        ApiVersion: api_version,
        Intents:    int(Intent_ALL),
        Partials:   partials,
        Debug:      debug,
        AllReady:   false,
      },
      Shards:       shards,
    },
  }
}

// Client Login
func (c *Client) Login(token string) {
  
  // set client token
  c.session.Data.Token = token
  // concurrency channles
  c.session.Done = make(chan bool)
	interrupt := make(chan os.Signal, 1)
  signal.Notify(interrupt, os.Interrupt)
  
  // loop shards
  for i := 1; i <= c.session.Shards; i++ {
    go c.dialGatway(c.session.Done, c.session.Data.ApiVersion, c.session.Data.Token, i)
  }

  // keep alive
  for {
    select {
      case<-interrupt:
        return
      default:
        continue
    }
  }

}

// callback intializers
func (c *Client) OnDebug(cb func(client *Client, message string)) { c.cbDebug = cb }
func (c *Client) OnReady(cb func(client *Client)) { c.cbReady = cb }
func (c *Client) OnGuildCreate(cb func(client *Client, guild *Guild)) { c.cbGuildCreate = cb }
func (c *Client) OnGuildUpdate(cb func(client *Client, guild *Guild, old_guild Guild)) { c.cbGuildUpdate = cb }
func (c *Client) OnGuildDelete(cb func(client *Client, guild Guild)) { c.cbGuildDelete = cb }
func (c *Client) OnChannelCreate(cb func(client *Client, channel *Channel)) { c.cbChannelCreate = cb }
func (c *Client) OnChannelUpdate(cb func(client *Client, channel *Channel, old_channel Channel)) { c.cbChannelUpdate = cb }
func (c *Client) OnChannelDelete(cb func(client *Client, channel Channel)) { c.cbChannelDelete = cb }
func (c *Client) OnMessageCreate(cb func(client *Client, message *Message)) { c.cbMessageCreate = cb }
func (c *Client) OnMessageUpdate(cb func(client *Client, message *Message, old_message Message)) { c.cbMessageUpdate = cb }
func (c *Client) OnMessageDelete(cb func(client *Client, message Message)) { c.cbMessageDelete = cb }
