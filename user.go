package discord

// client user
type clientUser struct {
  Id        string 
  Username  string 
}
// user struct
type User struct {
    Id          string
    Username    string
    Bot         bool
}
// user manager
type userManager struct { users map[string]*User }
func (manager *userManager) Get(user_id string) *User {
  return manager.users[user_id]
}
