package discord

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/signal"
	"runtime"
	"time"

	"github.com/gorilla/websocket"
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


func identify(conn *websocket.Conn, token string) {
	identify_payload := sendPaylod{
		Opcode: 2,
		Data: map[string]any{
			"token":   		token,
			"intents": 		Intent_ALL,
			"properties": 	map[string]any{
				"$os":      runtime.GOOS,
				"$browser": "github.com/grhx/disgord",
				"$device":  "golang",
			},
		},
	}
	json_msg, err := json.Marshal(identify_payload)
	if err != nil {
		log.Fatal(err)
	}
	conn.WriteMessage(websocket.TextMessage, json_msg)
}

func heartbeat(ch chan bool, conn *websocket.Conn) {
	defer close(ch)
	hb_msg, err := json.Marshal(sendPaylod{Opcode: 1,Data:nil})
	if err != nil {
		log.Fatal(err)
	}
	conn.WriteMessage(websocket.TextMessage, hb_msg)
	for {
		time.Sleep(time.Second * 40)
		conn.WriteMessage(websocket.TextMessage, hb_msg)
	}
}

func (c *Client) dialGateway(ch chan bool, token string) {
	defer close(ch)

	interrupt := make(chan os.Signal, 1)
	done := make(chan bool)
	signal.Notify(interrupt, os.Interrupt)
	conn, _, err := websocket.DefaultDialer.Dial("wss://gateway.discord.gg/?v=10&encoding=json", nil)
	if err != nil {
		log.Fatal(err)
	}
	go func() {
		defer close(done)
		for {
			_, message, err := conn.ReadMessage()
			if err != nil {
				log.Println("read:", err)
				return
			}
			var payload payload
			err = json.Unmarshal(message, &payload)
			if err != nil {
				log.Fatal(err)
			}
			switch Opcode(payload.Opcode) {
			// dipatch
			case Opcode_DISPATCH:

				// debug //
				times := time.Now()
				log_debug := fmt.Sprintf("\x1b[42m%d/%d/%d %d:%d:%d LOG:\x1b[0m", times.Year(), times.Month(), times.Day(), times.Hour(), times.Minute(), times.Second())
				event_debug := fmt.Sprintf("%s %s", log_debug, fmt.Sprintf("op: %d, t: %s, s: %d", payload.Opcode, payload.Type, payload.Sequence))
				fmt.Println(event_debug)
				///////////
				// send log msg //
				if payload.Type != "MESSAGE_CREATE" {
					go httpMesageCreate(c, "1161886243123122186", fmt.Sprintf("t: %s, s: %d", payload.Type, payload.Sequence))
				}
				/////////////////

				switch Event(payload.Type) {
				case Event_READY:
					// ready payload data
					var readyData readyPayload
					err = json.Unmarshal(message, &readyData)
					if err != nil { log.Fatal(err) }
					fmt.Printf("\nREADY DATA\n%v\n\n", readyData)
					// set client user
					c.User = *readyData.Data.User
					// ready callback
					go c.onReady(c)
				case Event_MESSAGE_CREATE:
					fmt.Println("HERE")
					// c.onMessageCreate(c, &msg)
				case Event_GUILD_CREATE:
				}
			// heartbeat
			case Opcode_HEARTBEAT:
			// reconnect
			case Opcode_RECONNECT:
			// invalid session
			case Opcode_INVALID_SESSION:
			// hello
			case Opcode_HELLO:
				identify(conn, token)
				go heartbeat(ch, conn)
			// heartbeat acknowledge
			case Opcode_HEARTBEAT_ACK:
			}
		}
	}()

	for {
		select {
		case <-done:
			return
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