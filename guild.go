package discord



// guild struct
type Guild struct {
  Id    string  `json:"id"`
}
// guild manager
type guildManager struct { guilds map[string]*Guild }
func (manager *guildManager) Add(guild *Guild) {
  manager.guilds[guild.Id] = guild
}
