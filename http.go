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
    DISCORD_API string = "https://discord.com/api"
)

// message requests
func (c *Client) httpMessageCreate(channel_id string, message Message) {
    
    // create body
    postBody, _ := json.Marshal(map[string]string{
        "content": "test",
    })
    // convert to reader
    reqBody := strings.NewReader(string(postBody))
    // create request
    req, err := http.NewRequest("POST", fmt.Sprintf("https://discord.com/api/v10/channels/%s/messages", channel_id), reqBody)
    if err != nil { log.Fatal(err) }
    fmt.Printf("TOKEN: %s\n", c.session.Data.Token)
    // add headers
    req.Header.Add("Authentication", fmt.Sprintf("Bot %s", c.session.Data.Token))
    req.Header.Add("Content-Type", "application/json")
    req.Header.Add("Accept", "application/json")
    // call request
    res, err := http.DefaultClient.Do(req)
    if err != nil { log.Fatal(err) }
    
    resBody, err := io.ReadAll(res.Body)
	if err != nil { log.Fatal(err) }
	fmt.Printf("%s\n", resBody)

}
