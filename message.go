package discord

// message struct
type Message struct {
    Id                  string      `json:"id"`
    Type                int         `json:"type"` // add constants
    Content             string      `json:"content"`
    Channel             *Channel    `json:"_"`
    Author              *User       `json:"_"`
    Embeds              []any       `json:"_"`
    Attachments         []any       `json:"_"`
    Components          []any       `json:"_"`
    Mentions            []any       `json:"_"` // pointer member array
    MentionRoles        []any       `json:"_"` // pointer role array
    Pinned              bool        `json:"pinned"`
    MentionEveryone     bool        `json:"mention_everyone"`
    Tts                 bool        `json:"tts"`
    Timestamp           string      `json:"timestamp"` // maybe add date / time type
    EditedTimestamp     string      `json:"edited_timestamp"` // same ^^^
    Flags               int         `json:"flags"` // add constants
    ReferencedMessage   any         `json:"_"` // pointer message
}

// message functions
func (message *Message) Delete() {}
func (message *Message) Reply(msg Message) { message.Channel.cRef.httpMessageCreateReply(message, msg) }
