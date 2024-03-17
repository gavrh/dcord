package discord

import (
	"runtime"
)

// init client func
func Init(intents []Intent, partials int, api_version int, shards int) *Client {
	return &Client{
		Users: UserManager{users: make(map[string]*User)},
		Guilds: GuildManager{guilds: make(map[string]*Guild)},
		Channels: ChannelManager{channels: make(map[string]*Channel)},
		Session: Session{
			ApiVersion: api_version,
			GatewayVersion: api_version,
			Data: SessionData{
				OS: runtime.GOOS,
				Intents: int(Intent_ALL),
				Partials: partials,
				EventCallbacks: map[string]any{},
			},
			Shards: shards,
		},
	}
}