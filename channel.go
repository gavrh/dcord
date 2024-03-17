package discord

// channel struct
type Channel struct {
	Id      string
	GuildId string
}

// channel manager struct
type ChannelManager struct {
	channels map[string]*Channel
}

func (channels ChannelManager) Add(channel *Channel) {
	channels.channels[channel.Id] = channel
}
func (channels ChannelManager) Get(cid string) *Channel {
	return channels.channels[cid]
}
func (channels ChannelManager) Delete(cid string) {
	delete(channels.channels, cid)
}
