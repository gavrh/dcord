package discord

// message struct
type Message struct {
	Id      string		`json:"id"`
	Channel *Channel
	Content string		`json:"content"`
}
