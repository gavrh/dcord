package discord

import (
    "net/http"
    "strings"
    "encoding/json"
    "fmt"
    "log"
)

const (
    discord_api string = "https://discord.com/api"
)

// message requests
func (c *Client) httpSendMessage(reqBody *strings.Reader, channel_id string) {
    req, _ := http.NewRequest("POST", fmt.Sprintf("%s/v10/channels/%s/messages", discord_api, channel_id), reqBody)

    req.Header.Add("Authorization", fmt.Sprintf("Bot %s", c.session.Data.Token))
    req.Header.Add("Content-Type", "application/json")
    req.Header.Add("Accept", "application/json")

    _, err := http.DefaultClient.Do(req)
    if err != nil { log.Fatal(err) }
}
func (c *Client) httpMessageCreate(channel_id string, message Message) {
    
    postBody, _ := json.Marshal(map[string]string{
        "content": message.Content,
    })
    reqBody := strings.NewReader(string(postBody))
    c.httpSendMessage(reqBody, channel_id)
}
func (c *Client) httpMessageCreateReply(ref_message *Message, message Message) {
    
    postBody, _ := json.Marshal(map[string]any{
        "content": message.Content,
        "message_reference": map[string]string{
            "message_id": ref_message.Id,
        },
    })
    reqBody := strings.NewReader(string(postBody))
    c.httpSendMessage(reqBody, ref_message.Channel.Id)

}
