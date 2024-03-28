package discord

import (
    "fmt"
)

// temp
type ChannelType int 
const (   
    ChannelType_GUILD_TEXT          ChannelType = 0
    ChannelType_DM                  ChannelType = 1
    ChannelType_GUILD_VOICE         ChannelType = 2
    ChannelType_GROUP_DM            ChannelType = 3
    ChannelType_GUILD_CATEGORY      ChannelType = 4
    ChannelType_GUILD_ANNOUNCEMENT  ChannelType = 5
    ChannelType_ANNOUNCEMENT_THREAD ChannelType = 10
    ChannelType_PUBLIC_THREAD       ChannelType = 11
    ChannelType_PRIVATE_THREAD      ChannelType = 12
    ChannelType_GUILD_STAGE_VOICE   ChannelType = 13
    ChannelType_GUILD_DIRECTORY     ChannelType = 14
    ChannelType_GUILD_FORUM         ChannelType = 15
    ChannelType_GUILD_MEDIA         ChannelType = 16
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
     channel.cRef.httpMessageCreate(channel.Id, message)
}
