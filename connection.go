package discord

import (
  "github.com/gorilla/websocket"
  "fmt"
  "log"
  "os"
  "os/signal"
  "encoding/json"
)

// connection struct
type Connection struct {
  Socket      *websocket.Conn
  Shard       int
  SessionType string
  SessionId   string
  Sequence    int
}


// dial gateway
func dialGatway(ch chan bool, apiV int, token string, shard int) {
  defer close(ch)
  // concurrency channels
  done := make(chan bool)
  interrupt := make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt)
  // connect to gateway
  conn, _, err := websocket.DefaultDialer.Dial(fmt.Sprintf("wss://discord.gg/?v=%d&encoding=json", apiV), nil)
  if err != nil { log.Fatal(err) }
  // maintain connection 
  go maintainConn(done, conn, token, shard)
  // keep alive
  for {
    select {
    case<-done:
      return
    case<-interrupt:
      return
    default:
      continue
    }
  }
}
// temp payload struct 
type payload struct {
  Opcode    opcode
  Sequence  int
  Type      Event
  Data      map[string]any
}

// maintain connection 
func maintainConn(done chan bool, conn *websocket.Conn, token string, shard int) {
  defer close(done)
  for {
    // get ws message
    _, message, err := conn.ReadMessage()
    if err != nil {
      log.Fatal(err)
      return
    }
    // json to map
    var payload payload
    err = json.Unmarshal(message, &payload)
    if err != nil { log.Fatal(err) }

    fmt.Printf("%s", message)
    fmt.Printf("%v", payload)
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
      println("HELLO")
    // heartbeat
    case opcode_HEARTBEAT:
      println("HEARTBEAT")
    // heatbeat acknowledge
    case opcode_HEARTBEAT_ACK:
      println("HEARTBEAT_ACK")
    }
  }
}
