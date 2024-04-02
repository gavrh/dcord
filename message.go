package discord

// message struct
type Message struct {
    Id      string
    Content string
    Author  Member
    Metions []Member
}

// message functions
func (message *Message) Delete() {}
func (message *Message) Reply() {}
