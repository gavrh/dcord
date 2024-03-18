package discord

// guild stuct
type Guild struct {
	Id                          string
	Name                        string
	Icon                        string // optional
	Splash                      string // optional
	DiscoverySplash             string // optional
	Owner                       *User
	Permissions                 string
	Region                      string   // optional
	AfkChannel                  *Channel // optional
	AfkTimeout                  int
	WidgetEnabled               bool
	WidgetChannel               *Channel // optional
	VerificationLevel           int
	DefaultMessageNotifications int
	ExplicitContentFilter       int
	Roles                       map[string]*any // later
	Emojis                      map[string]*any // later
	Features                    []string
	MfaLevel                    int
	ApplicationId               int      // optional
	SystemChannel               *Channel // optional
	SystemChannelFlags          int
	RulesChannel                *Channel // optional
	MaxPresences                int      // optional
	MaxMembers                  int
	VanityUrlCode               string // optional
	Description                 string // optional
	Banner                      string // optional
	PremiumTier                 int
	PremiumSubscriptionCount    int
	PreferredLocale             string
	PublicUpdatesChannel        *Channel // optional
	MaxVideoChannelUsers        int
	MaxStageVideoChannelUsers   int
	ApproximateMemberCount      int
	Channels                    map[string]*Channel
}

// guild manager struct
type guildManager struct {
	guilds map[string]*Guild
}

func (guilds guildManager) Add(guild *Guild) {
	guilds.guilds[guild.Id] = guild
}
func (guilds guildManager) Get(gid string) *Guild {
	return guilds.guilds[gid]
}
