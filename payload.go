package discord

import (
    "encoding/json"
	"fmt"
	"log"
	"runtime"
	"time"
	"github.com/gorilla/websocket"
)

// receive payload
type payload struct {
    Opcode    opcode          `json:"op"`
    Sequence  int             `json:"s"`
    Type      Event           `json:"t"`
    Data      map[string]any  `json:"d"`
}

// event handler 
func (c *Client) handlePayload(conn *websocket.Conn, payload *payload, message *[]byte, token string, done chan bool) {

    fmt.Printf("\n%s\n", *message)

    // check opcode
    switch payload.Opcode { 
        // dispatch
        case opcode_DISPATCH:
            switch payload.Type {
                case Event_READY:
                    go c.handleReady(message)
                case Event_GUILD_CREATE:
                    go c.handleGuildCreate(message)
                case Event_MESSAGE_CREATE:
                    go c.handleMessageCreate(message)

            }
            println(payload.Type)
        // reconnect
        case opcode_RECONNECT:
            println("RECONNECT")
        // invalid session
        case opcode_INVALID_SESSION:
            println("INVALID_SESSION")
        // hello
        case opcode_HELLO:
            c.identify(conn, token)
            go heartbeat(done, conn)
            println("HELLO")
        // heartbeat
        case opcode_HEARTBEAT:
            println("HEARTBEAT")
        // heartbeat ack
        case opcode_HEARTBEAT_ACK:
            println("HEARTBEAT_ACK")
    }
}

// dispatch payloads
type readyData struct {
    ApiVerson             int             `json:"v"`
    UserSettings          map[string]any  `json:"user_settings"` 
    User                  clientUser      `json:"user"`
    SessionType           string          `json:"session_type"`
    SessionId             string          `json:"session_id"`
    GatewayResumeUrl      string          `json:"gateway_resume_url"`
    Relationships         []any           `json:"relationships"`
    PrivateChannels       []any           `json:"private_channels"`
    Presences             []any           `json:"presences"`
    Guilds                []any           `json:"guilds"`
    GuildJoinRequests     []any           `json:"guild_join_requests"`
    GeoOrderedRtcRegions  []any           `json:"geo_ordered_rtc_regions"`
    Auth                  map[string]any  `json:"auth"`
    Application           map[string]any  `json:"application"`
}

// handle ready
type readyPayload struct {
    Data  readyData `json:"d"`
}
func (c *Client) handleReady(message *[]byte) {
    var readyInfo readyPayload
    err := json.Unmarshal(*message, &readyInfo)
    if err != nil { log.Fatal(err) }
    
    c.User = &readyInfo.Data.User
    for ;; {
        if len(c.Guilds.guilds) == len(readyInfo.Data.Guilds) { break }
    }
    go c.cbReady(c)
    c.session.Data.AllReady = true
}

// handle guild
type guildCreateExtraData struct {
    SafteyAlertsChannelId   string      `json:"safety_alerts_channel_id"`
    PublicUpdatesChannelId  string      `json:"public_updates_channel_id"`
    SystemChannelId         string      `json:"system_channel_id"`
    AfkChannelId            string      `json:"afk_channel_id"`
    RulesChannelId          string      `json:"rules_channel_id"`
    Threads                 []*Channel  `json:"threads"`
    Channels                []*Channel  `json:"channels"`
    Members                 []*Member   `json:"members"`
}
type guildCreateExtraPayload struct {
    Data guildCreateExtraData `json:"d"`
}
type guildCreatePayload struct {
    Data Guild `json:"d"`
}
func (c *Client) handleGuildCreate(message *[]byte) {    
    
    var new_guild guildCreatePayload
    new_guild.Data.Channels = channelManager{ channels: make(map[string]*Channel) }
    new_guild.Data.Members = memberManager{ members: make(map[string]*Member) }
    var new_guild_extra guildCreateExtraPayload
    err := json.Unmarshal(*message, &new_guild)
    if err != nil { log.Fatal(err) }
    err = json.Unmarshal(*message, &new_guild_extra) 
    if err != nil { log.Fatal(err) }

    // fill in extra data
    for i := 0; i < len(new_guild_extra.Data.Channels); i++ {
        // add to channel manager of guild and client
        new_guild_extra.Data.Channels[i].cRef = c
        new_guild.Data.Channels.channels[new_guild_extra.Data.Channels[i].Id] = new_guild_extra.Data.Channels[i]
        c.Channels.channels[new_guild_extra.Data.Channels[i].Id] = new_guild_extra.Data.Channels[i]
    }
    new_guild.Data.SafetyAlertsChannel = new_guild.Data.Channels.channels[new_guild_extra.Data.SafteyAlertsChannelId]
    new_guild.Data.PublicUpdatesChannel = new_guild.Data.Channels.channels[new_guild_extra.Data.PublicUpdatesChannelId]
    new_guild.Data.SystemChannel = new_guild.Data.Channels.channels[new_guild_extra.Data.SystemChannelId]
    new_guild.Data.AfkChannel = new_guild.Data.Channels.channels[new_guild_extra.Data.AfkChannelId]
    new_guild.Data.RulesChannel = new_guild.Data.Channels.channels[new_guild_extra.Data.RulesChannelId]
    for i := 0; i < len(new_guild_extra.Data.Members); i++ {
        // add user and member to respective managers
        c.Users.users[new_guild_extra.Data.Members[i].User.Id] = new_guild_extra.Data.Members[i].User
        new_guild.Data.Members.members[new_guild_extra.Data.Members[i].User.Id] = new_guild_extra.Data.Members[i]
    }
    // initial guilds havent been cached yet
    if !c.session.Data.AllReady {
        c.Guilds.Add(&new_guild.Data)
        return
    }
    // callback
    go c.cbGuildCreate(c, &new_guild.Data)
}
func (c *Client) handleGuildUpdate(message *[]byte) {

    updated_guild := &Guild{}
    old_guild := Guild{}

    // callback
    go c.cbGuildUpdate(c, updated_guild, old_guild)
}
func (c *Client) handleGuildDelete(message *[]byte) {}

// handle message
type messageCreateExtraData struct {
    Author struct {
        Id  string `json:"id"`
    } `json:"author"`
    ChannelId   string  `json:"channel_id"` 
}
type messageCreateExtraPayload struct {
    Data messageCreateExtraData `json:"d"`
}
type messageCreatePayload struct {
    Data Message `json:"d"`
}
func (c *Client) handleMessageCreate(message *[]byte) {

    // create and unmarshal data
    var new_message messageCreatePayload
    var new_message_extra messageCreateExtraPayload
    println("new_guild_extra")
    err := json.Unmarshal(*message, &new_message)
    if err != nil { log.Fatal(err) }
    err = json.Unmarshal(*message, &new_message_extra)
    if err != nil { log.Fatal(err) }

    // fill extra data
    new_message.Data.Author = c.Users.users[new_message_extra.Data.Author.Id]
    new_message.Data.Channel = c.Channels.channels[new_message_extra.Data.ChannelId]

    // run callback
    go c.cbMessageCreate(c, &new_message.Data)
}
func (c *Client) handleMessageUpdate(message *[]byte) {}
func (c *Client) handleMessageDelete(message *[]byte) {}


// reconnect payload
// invalid session payload
// samples to ge up and running
func (c *Client) identify(conn *websocket.Conn, token string) {
    // identify payload
    payload, err := json.Marshal(payload{
        Opcode: opcode_IDENTIFY,
        Data: map[string]any{
            "token": token,
            "intents": c.session.Data.Intents,
            "properties": map[string]any{
                "$os": runtime.GOOS,
                "$browser": "github.com/grhx/disgord",
                "$device": "golang",
            },
        },
    })
    if err != nil { log.Fatal(err) }
    conn.WriteMessage(websocket.TextMessage, payload)
}

func heartbeat(done chan bool, conn *websocket.Conn) {
    // heartbeat payload
    payload, err := json.Marshal(payload{Opcode:opcode_HEARTBEAT})
    if err != nil { log.Fatal(err) }
    conn.WriteMessage(websocket.TextMessage, payload)
    for {
        select {
            case<-done:
                return
            default:
                time.Sleep(time.Second * 40)
                conn.WriteMessage(websocket.TextMessage, payload)
        }
    }
}
