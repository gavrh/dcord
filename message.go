package discord

import "fmt"

// message struct
type Message struct {
	Id                  	string
	ChannelId           	string
	Author              	any
	Content             	string
	Timestamp           	string
	EditedTimestamp     	string
	Tts                 	bool
	MentionEveryone     	bool
	Mentions            	[]any
	MentionRoles        	[]any
	MentionChannels     	[]any
	Attachments         	[]any
	Embeds              	[]any
	Reactions           	[]any
	Nonce               	any
	Pinned              	bool
	WebhookId           	string
	Type                	string
	Activity            	any
	Application         	any
	ApplicationId       	any
	MessageReference    	any
	Flags               	any
	ReferencedMessage   	any
	Interactions        	any
	Thread              	any
	Components          	any
	StickerItems        	[]any
	Stickers            	[]any
	Position            	int
	RoleSubscriptionData	any
	Resolved            	any
}

// extract message from event data
func extractMessage(data map[string]any) Message {

	fmt.Printf("%s\n", data)

	return Message{
		Id:        fmt.Sprint(data["id"]),
		Content:   fmt.Sprint(data["content"]),
		ChannelId: fmt.Sprint(data["channel_id"]),
	}

}
