package discord

// client user
type ClientUser struct {}
// user struct
type User struct {}
// user manager
type userManager struct { users map[string]*User }
func (manager *userManager) Get(user_id string) *User {
  return manager.users[user_id]
}
