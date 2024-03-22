package discord

import (
  "github.com/gorilla/websocket"
  "log"
  "runtime"
  "encoding/json"
  "time"
)

// receive payload
type payload struct {
  Opcode    opcode          `json:"op"`
  Sequence  int             `json:"s"`
  Type      Event           `json:"t"`
  Data      map[string]any  `json:"d"`
}

// event handler 
func handlePayload(client *Client, conn *websocket.Conn, payload *payload, token string, done chan bool) {

  // check opcode
  switch payload.Opcode { 
    // dispatch
    case opcode_DISPATCH:
      println(payload.Type)
    // reconnect
    case opcode_RECONNECT:
      println("RECONNECT")
    // invalid session
    case opcode_INVALID_SESSION:
      println("INVALID_SESSION")
    // hello
    case opcode_HELLO:
      identify(client, conn, token)
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
// all dispatch payloads
// samples to ge up and running
func identify(client *Client, conn *websocket.Conn, token string) {
  // identify payload
  payload, err := json.Marshal(payload{
    Opcode: opcode_IDENTIFY,
    Data: map[string]any{
      "token": token,
      "intents": client.session.Data.Intents,
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
  payload, err := json.Marshal(payload{Opcode:opcode_HEARTBEAT,Data:nil})
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
