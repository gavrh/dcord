package discord

import (
    "fmt"
)

// channel struct
type Channel struct {
    Id      string
    Name    string
    Type    ChannelType

    cRef    *Client
}
// channel manager
type channelManager struct { channels map[string]*Channel }
func (manager *channelManager) Size() int {
    return len(manager.channels)
}
func (manager *channelManager) ForEach(function func(c *Channel)) {
    for _, ch := range manager.channels {
        function(ch)
    }
}
func (manager *channelManager) PrintAll() {
    for _, c := range manager.channels {
        fmt.Printf("%#v\n", c)
    }
}
// channel functions
func (channel *Channel) Send(message Message) {
     go channel.cRef.httpMessageCreate(channel.Id, message)
}
