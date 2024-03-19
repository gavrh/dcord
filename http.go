package discord

import (
	"strings"
	"encoding/json"
	"fmt"
	"log"
	"io/ioutil"
	"net/http"
)

// message create
func httpMesageCreate(c *Client, channel_id string, content string) {

	postBody, _ := json.Marshal(map[string]string{
		"content": content,
	})
	reqBody := strings.NewReader(string(postBody))
	req, err := http.NewRequest("POST", fmt.Sprintf("https://discord.com/api/v10/channels/%s/messages", channel_id), reqBody)
	if err != nil { log.Fatal(err) }
	req.Header.Add("Authorization", fmt.Sprintf("Bot %s", c.Session.Data.Token))
	req.Header.Add("Content-Type", "application/json")
	println("YADA TOKEN " + c.Session.Data.Token)
	res, err := http.DefaultClient.Do(req)
	if err != nil { log.Fatal(err) }
  
  

	resBody, err := ioutil.ReadAll(res.Body)
	if err != nil { log.Fatal(err) }
	fmt.Printf("%s", resBody)

	
}
