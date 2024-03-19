package discord

import (
	"runtime"
)

// init client func
func Init(intents []Intent, partials int, api_version int, shards int, cache bool) *Client {
	return &Client{
		Users: userManager{users: make(map[string]*User)},
		Guilds: guildManager{guilds: make(map[string]*Guild)},
		Channels: channelManager{channels: make(map[string]*Channel)},
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