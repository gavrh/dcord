package discord

import (
    "net/http"
    "strings"
    "encoding/json"
    "fmt"
    "log"
    "io"
)

const (
    discord_api string = "https://discord.com/api"
)

// message requests
func (c *Client) httpMessageCreate(channel_id string, message Message) {
    
    // create body
    postBody, _ := json.Marshal(map[string]string{
        "content": message.Content,
    })
    // convert to reader
    reqBody := strings.NewReader(string(postBody))
    // create request
    fmt.Printf("%v\n",*reqBody)
    req, err := http.NewRequest("POST", fmt.Sprintf("%s/v10/channels/%s/messages", discord_api, channel_id), reqBody)
    if err != nil { log.Fatal(err) }
    fmt.Printf("TOKEN: %s\n", c.session.Data.Token)
    // add headers
    req.Header.Add("Authorization", fmt.Sprintf("Bot %s", c.session.Data.Token))
    req.Header.Add("Content-Type", "application/json")
    req.Header.Add("Accept", "application/json")
    // call request
    res, err := http.DefaultClient.Do(req)
    if err != nil { log.Fatal(err) }
    
    resBody, err := io.ReadAll(res.Body)
	if err != nil { log.Fatal(err) }
	fmt.Printf("%s\n", resBody)

}
