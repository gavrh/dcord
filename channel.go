package discord

// channel struct
type Channel struct {
	Id      string
	GuildId string
}

// channel manager struct
type channelManager struct {
	channels map[string]*Channel
}

func (channels channelManager) Add(channel *Channel) {
	channels.channels[channel.Id] = channel
}
func (channels channelManager) Get(cid string) *Channel {
	return channels.channels[cid]
}
func (channels channelManager) Delete(cid string) {
	delete(channels.channels, cid)
}
