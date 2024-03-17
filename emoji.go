package discord

// emoji struct
type Emoji struct {
	Id            string
	Name          string
	Roles         []string
	User          *User
	RequireColons bool
	Managed       bool
	Animated      bool
	Available     bool
}
