package discord

import (
	"encoding/json"
    "fmt"
	"os"
	"os/signal"
    "runtime"
    "time"

	"github.com/gorilla/websocket"
)

// client struct
type Client struct {
    User                            *clientUser
    Users                           userManager
    Channels                        channelManager
    Guilds                          guildManager

    session                         session
    connections                     connectionManager

    cbDebug                         func(*Client, string)
    cbReady                         func(*Client)
    cbGuildCreate                   func(*Client, *Guild)
    cbGuildUpdate                   func(*Client, *Guild, Guild)
    cbGuildDelete                   func(*Client, Guild)
    cbGuildRoleCreate               func(*Client, *Role)
    cbGuildRoleUpdate               func(*Client, *Role, Role)
    cbGuildRoleDelete               func(*Client, Role)
    cbChannelCreate                 func(*Client, *Channel)
    cbChannelUpdate                 func(*Client, *Channel, Channel)
    cbChannelDelete                 func(*Client, Channel)
    cbChannelPinsUpdate             func(*Client)
    cbThreadCreate                  func(*Client)
    cbThreadUpdate                  func(*Client)
    cbThreadDelete                  func(*Client)
    cbThreadMemberSync              func(*Client)
    cbThreadMemberUpdate            func(*Client)
    cbThreadMembersUpdate           func(*Client)
    cbStageInstanceCreate           func(*Client)
    cbStageInstanceUpdate           func(*Client)
    cbStageInstanceDelete           func(*Client)
    cbGuildMemberAdd                func(*Client, *Member)
    cbGuildMemberUpdate             func(*Client, *Member, Member)
    cbGuildMemberRemove             func(*Client, Member)
    cbGuildAuditLogEntryCreate      func(*Client)
    cbIntegrationCreate             func(*Client)
    cbIntegrationUpdate             func(*Client)
    cbIntegrationDelete             func(*Client)
    cbWebhooksUpdate                func(*Client)
    cbInviteCreate                  func(*Client)
    cbInviteDelete                  func(*Client)
    cbVoiceStateUpdate              func(*Client)
    cbPresenceUpdate                func(*Client)
    cbMessageCreate                 func(*Client, *Message)
    cbMessageUpdate                 func(*Client, *Message, Message)
    cbMessageDelete                 func(*Client, Message)
    cbMessageDeleteBulk             func(*Client)
    cbMessageReactionAdd            func(*Client)
    cbMessageReactionRemove         func(*Client)
    cbMessageReactionRemoveAll      func(*Client)
    cbMessageReactionRemoveEmoji    func(*Client)
    cbTypingStart                   func(*Client)
    cbGuildScheduledEventCreate     func(*Client)
    cbGuildScheduledEventUpdate     func(*Client)
    cbGuildScheduledEventDelete     func(*Client)
    cbGuildScheduledEventUserAdd    func(*Client)
    cbGuildScheduledEventUserRemove func(*Client)
    cbAutoModerationRuleCreate      func(*Client)
    cbAutoModerationRuleUpdate      func(*Client)
    cbAutoModerationRuleDelete      func(*Client)
    cbAutoModerationActionExecution func(*Client)

}

func Init(intents []Intent, partials int, api_version int, shards int, debug bool) Client {
    return Client{
        Users:          userManager{users: make(map[string]*User)},
        Channels:       channelManager{channels: make(map[string]*Channel)},
        Guilds:         guildManager{guilds: make(map[string]*Guild)},

        session: session{
            Data:       sessionData{
            OS:         runtime.GOOS,
            ApiVersion: api_version,
            Intents:    int(Intent_ALL),
            Partials:   partials,
            Debug:      debug,
            AllReady:   false,
            },
            Shards:         shards,
        },
        connections:   connectionManager{connections: make(map[string]*connection)},
    }
}

func (c *Client) Login(token string) {

    // set client token
    c.session.Data.Token = token
    // concurrency channles
    c.session.Done = make(chan bool)
	interrupt := make(chan os.Signal, 1)
    signal.Notify(interrupt, os.Interrupt)

    // loop shards
    for i := 1; i <= c.session.Shards; i++ {
        go c.dialGateway(c.session.Done, c.session.Data.ApiVersion, c.session.Data.Token, i)
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

// temp
type Activity struct {
    Name        string  `json:"name"`
    Type        int     `json:"type"`
    CreatedAt   int     `json:"created_at"`
    // add rest later
}
func (c *Client) SetPresence() {
    c.connections.ForEach(func(conn *connection, key string) {
        fmt.Printf("%v\n", conn)
        presence, _ := json.Marshal(map[string]any{
            "op": opcode_PRESENCE_UPDATE,
            "d": map[string]any{
                "activities": []Activity{
                    {
                        Name: "Privacy",
                        Type: 5,
                        CreatedAt: int(time.Now().UnixMilli()),
                    },

                },
                "since": int(time.Now().UnixMilli()),
                "status": "idle",
                "afk": false,
            },
        })
        println(string(presence))
        conn.Socket.WriteMessage(websocket.TextMessage, presence)
    })
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
func (c *Client) OnGuildMemberAdd(cb func(client *Client, member *Member)) { c.cbGuildMemberAdd = cb }
func (c *Client) OnGuildMemberUpdate(cb func(client *Client, member *Member, old_member Member)) { c.cbGuildMemberUpdate = cb }
func (c *Client) OnGuildMemberDelete(cb func(client *Client, member Member)) { c.cbGuildMemberRemove = cb }
func (c *Client) OnMessageCreate(cb func(client *Client, message *Message)) { c.cbMessageCreate = cb }
func (c *Client) OnMessageUpdate(cb func(client *Client, message *Message, old_message Message)) { c.cbMessageUpdate = cb }
func (c *Client) OnMessageDelete(cb func(client *Client, message Message)) { c.cbMessageDelete = cb }
