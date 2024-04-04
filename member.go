package discord

// member struct
type Member struct {
    User    *User    `json:"user"`
}

// member manager
type memberManager struct { members map[string]*Member }
func (manager *memberManager) Add(member *Member) {
    manager.members[member.User.Id] = member
}
