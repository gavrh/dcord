package discord

import (
    "encoding/json"
	"fmt"
	"log"
	"github.com/gorilla/websocket"
)

// connection struct
type connection struct {
    Socket      *websocket.Conn
    Shard       int
    SessionType string
    SessionId   string
    Sequence    int
}
// connection manager
type connectionManager struct {
    connections map[string]*connection
}


// dial gateway
func (c *Client) dialGateway(cChan chan bool, apiV int, token string, shard int) {
    defer close(cChan) 
    done := make(chan bool)
    
    socket, _, err := websocket.DefaultDialer.Dial(fmt.Sprintf("wss://gateway.discord.gg/?v=%d&encoding=json", apiV), nil)
    if err != nil {
        println("error connecting to gateway")
        log.Fatal(err)
    }
    conn := &connection{ Socket: socket}
    
    go c.maintainConn(done, conn, token)
    for {
        select {
        case<-done:
            return
        default:
            continue
        }
    }
}

// maintain connection
func (c *Client) maintainConn(done chan bool, conn *connection, token string) {
    defer close(done)
    for {
        // get ws message
        _, message, err := conn.Socket.ReadMessage()
        if err != nil {
            log.Fatal(err)
            return
        }
        // json to map
        var payload payload
        err = json.Unmarshal(message, &payload)
        if err != nil { log.Fatal(err) }
        // handle payload
        c.handlePayload(conn, &payload, &message, token, done)
    }
}

