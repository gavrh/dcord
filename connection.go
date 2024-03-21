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
func dialGatway(ch chan, apiV int, token string, shard int) {
  
  // concurrency channels
  done := make(chan bool)
  interrupt := make(chan bool)
  interrupt := make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt)
  // connect to gateway
  conn, _, err := websocket.Dial(fmt.Sprintf("wss://discord.gg/?v=%d&encoding=json", apiV))
  if err != nil { log.Fatal(err) }
  // maintain gateway
  go maintainConn(done, conn)

  // loop until done
  for {
    select {
    case<-done:
      return
    case<-interrupt
      return
    default:
      continue
    }
  }
}
// maintain connection 
func maintainConn(done chan bool, conn *websocket.Conn) {
  defer(done)
  
  for {
    // get ws message
    _, message, err := conn.ReadMessage()
    if err != nil {
      log.Fatal(err)
      return
    }
    // json to map object
    var paylaod map[string]any
    err = json.Marshal(message, &paylaod)
    if err != nil { log.Fatal(err) }
      
    // check opcode
    switch payload["op"] {
    // dispatch
    case opcode_DISPATCH:
    // reconnect
    case opcode_RECONNECT:
    // invalid session 
    case opcode_INVALID_SESSION:
    // hello 
    case opcode_HELLO:
    // heartbeat
    case opcode_HEARTBEAT: 
    // heartbeat ack
    case opcode_HEARTBEAT_ACK:
    

    
    }
  }

}


