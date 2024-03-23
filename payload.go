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
func (c *Client) handlePayload(conn *websocket.Conn, payload *payload, message []byte, token string, done chan bool) {

  // check opcode
  switch payload.Opcode { 
    // dispatch
    case opcode_DISPATCH:
      switch payload.Type {
        case Event_READY:
          c.handleReady(message)
        case Event_GUILD_CREATE:
          c.handleGuildCreate(payload)

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
  User                  ClientUser      `json:"user"`
  SessionType           string          `json:"session_type"`
  SessionId             string          `json:"session_id"`
  GatewayResumeUrl      string          `json:"gateway_resume_url"`
  Relationships         []any           `json:"relationships"`
  PrivateChannels       []any           `json:"private_channels"`
  Presences             []any           `json:"presences"`
  Guilds                []any           `json:"guils"`
  GuildJoinRequests     []any           `json:"guild_join_requests"`
  GeoOrderedRtcRegions  []any           `json:"geo_ordered_rtc_regions"`
  Auth                  map[string]any  `json:"auth"`
  Application           map[string]any  `json:"application"`
}
type readyPayload struct {
  Data  readyData `json:"d"`
}
func (c *Client) handleReady(message []byte) {
  var readyInfo readyPayload
  err := json.Unmarshal(message, &readyInfo)
  if err != nil { log.Fatal(err) }
  fmt.Printf("READY PAYLOAD: %v\n", readyInfo.Data)
  c.User = &readyInfo.Data.User
  go c.cbReady(c)
}
func (c *Client) handleGuildCreate(payload *payload) {    
  new_guild := Guild{
    Id: "NEW_GUILD_ID",
  }
  c.Guilds.Add(new_guild.Id, &new_guild)
  go c.cbGuildCreate(c, &new_guild)
}
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
