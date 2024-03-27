package discord

import (
    "encoding/json"
	"fmt"
	"log"
	"github.com/gorilla/websocket"
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
func (c *Client) dialGatway(cChan chan bool, apiV int, token string, shard int) {
    defer close(cChan) 

    // concurrency channels
    fmt.Printf("Shard %d: Dialing Discord Gateway\n", shard)

    done := make(chan bool)
    // connect to gateway
    conn, _, err := websocket.DefaultDialer.Dial(fmt.Sprintf("wss://gateway.discord.gg/?v=%d&encoding=json", apiV), nil)
    if err != nil {
        println("error connecting to gateway")
        log.Fatal(err)
    }
    // maintain connection 
    go c.maintainConn(done, conn, token)
    // keep alive
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
func (c *Client) maintainConn(done chan bool, conn *websocket.Conn, token string) {
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
        // handle payload
        c.handlePayload(conn, &payload, &message, token, done)
    }
}

