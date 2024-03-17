package discord

import (
	"github.com/gorilla/websocket"
	"log"
	"sync"
	"os/signal"
	"os"
	"encoding/json"
	"fmt"
)

// # Connection Struct
type Connection struct {
	Socket		*websocket.Conn
	Shard 		int
	SessionId	string
	Sequence	int
	Timer		int
	Timestamp	int
	Buffers		int
	Heartbeats	int
}

func (c *Client) dialGateway() {

	var wg sync.WaitGroup
	// upgrader
	// var upgrader = websocket.Upgrader{
	// 	ReadBufferSize: 1024,
	// 	WriteBufferSize: 1024,
	// }
	// // handler
	// func handler(w http.http.ResponseWriter, r *http.Request) {

	// 	conn, err := upgrader.Upgrade(w, r, nil)
	// 	if err != nil {
	// 		log.Fatal(err)
	// 	}

	// 	return
	// }

	interrupt := make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt)
	conn, _, err := websocket.DefaultDialer.Dial("wss://gateway.discord.gg/?v=10&encoding=json", nil)
	if err != nil {
		log.Fatal(err)
	}
	go func() {
		defer wg.Done()
		for {
			_, message, err := conn.ReadMessage()
			if err != nil {
				log.Println("read:", err)
				return
			}
			log.Printf("recv: %s", message)
			var event map[string]any
			err = json.Unmarshal(message, &event)
			if err != nil {
				log.Fatal(err)
			}
			fmt.Printf("%v\n\n", event)
			switch event["op"] {
			case 11:
				println("heartbeat ack")
			case 10:
				
				println("HEARTBEAT STARTED")
				
			case 9:
			}

		}
	}()

	for {
		select {
		case <-interrupt:
			log.Println("interrupt")

			// Cleanly close the connection by sending a close message and then
			// waiting (with timeout) for the server to close the connection.
			err := conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""))
			if err != nil {
				log.Println("write close:", err)
				return
			}
		}
	}

}