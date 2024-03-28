package discord

// message struct
type Message struct {
    Id      string
    Content string
}

// message functions
func (message *Message) Delete() {}
func (message *Message) Reply() {}
