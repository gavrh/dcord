package discord

import "fmt"

// guild struct
type Guild struct {
    Id                          string              `json:"id"`
    Name                        string              `json:"name"`
    SafetyAlertsChannel         *Channel            `json:"_"`
    GuildScheduledEvents        []any               `json:"guild_scheduled_events"` // add later
    Members                     []any               `json:"_"` // []*Member
    Splash                      string              `json:"splash"`
    MaxVideoChannelUsers        int                 `json:"max_video_channel_users"`
    MfaLevel                    int                 `json:"mfa_level"`
    VanityUrlCode               string              `json:"vanity_url_code"`
    Stickers                    []any               `json:"stickers"` // []*Sticker
    ActivityInstances           map[string]any      `json:"activity_instances"`
    EmbeddedActivities          []any               `json:"embedded_activities` // add later
    ApplicationCommandCounts    map[string]any      `json:"application_command_counts"` // add later
    MaxStageVideoChannelUsers   int                 `json:"max_stage_video_channel_users"`
    DiscoverySplash             string              `json:"discovery_splash"`
    LatestOnboardingQuestionId  string              `json:"latest_onboarding_question_id`
    PublicUpdatesChannel        *Channel            `json:"_"`
    SystemChannelFlags          int                 `json:"system_channel_flags"`
    InventorySettings           any                 `json:"inventory_settings"` // add later
    Presences                   []any               `json:"presences"` // add later 
    Features                    []string            `json:"features"`
    PremiumProgressBarEnabled   bool                `json:"premium_progress_bar_enabled"`
    SystemChannel               *Channel            `json:"_"`
    Threads                     any                 `json:"_"`
    Channels                    channelManager      `json:"_"`
    PremiumSubscriptionCount    int                 `json:"premium_subscription_count"`
    DefaultMessageNotifications int                 `json:"default_message_notifications"`
    Description                 string              `json:"descritpion"`
    Unavailable                 bool                `json:"unavailable"`
    Banner                      string              `json:"banner"`
    VerificationLevel           int                 `json:"verification_level"`
    Emojis                      []any               `json:"_"` // []*Emoji
    MemberCount                 int                 `json:"member_count"`
    AfkTimeout                  int                 `json:"aft_timeout"`
    ExplicitContentFilter       int                 `json:"explicit_conten_filter"`
    AfkChannel                  *Channel            `json:"_"`
    SoundboardSounds            []any               `json:"soundboard_sounds"` // add later
    NsfwLevel                   int                 `json:"nsfw_level"`
    PreferredLocale             string              `json:"preferred_locale"`
    ApplicationId               string              `json:"application_id"`
    Nsfw                        bool                `json:"nsfw"`
    VoiceStates                 []any               `json:"voice_states"` // add later
    Icon                        string              `json:"icon"`
    Large                       bool                `json:"large"`
    PremiumTier                 int                 `json:"premium_tier"`
    MaxMembers                  int                 `json:"max_members"`
    Roles                       []*Role             `json:"_"`
    JoinedAt                    string              `json:"joined_at"`
    HomeHeader                  string              `json:"home_header"`
    Owner                       string              `json:"_"` // *Member
    Version                     int                 `json:"version"`
    IncidentsData               any                 `json:"incidents_data"` // add later
    Clan                        any                 `json:"clan"` // add later
    Lazy                        bool                `json:"lazy"`
    RulesChannel                *Channel            `json:"_"`
    StageInstances              []any               `json:"stage_instances"` // add later
    HubType                     any                 `json"hub_type"` // add later
}
// guild manager
type guildManager struct { guilds map[string]*Guild }
func (manager *guildManager) PrintAll() {
    for _, g := range manager.guilds {
        fmt.Printf("%#v\n", g)
    }
}
func (manager *guildManager) Size() int {
    return len(manager.guilds)
}
func (manager *guildManager) Add(guild *Guild) {
    manager.guilds[guild.Id] = guild
}
func (manager *guildManager) Get(guild_id string) *Guild {
    return manager.guilds[guild_id]
}
func (manager *guildManager) Replace(guild_id string, new_guild *Guild) {
    manager.guilds[guild_id] = new_guild
}
func (manager *guildManager) Remove(guild_id string) {
    delete(manager.guilds, guild_id)
}
func (manager *guildManager) Fetch(guild_id string) {
  
}
