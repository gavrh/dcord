package discord

// client user
type clientUser struct {
    Id        string 
    Username  string 
}
// user struct
type User struct {
    Id                      string  `json:"id"`
    Username                string  `json:"username"`
    GlobalName              string  `json:"global_name"`
    Discriminator           string  `json:"discriminator"`
    Bot                     bool    `json:"bot"`
    AccentColor             string  `json:"accent_color"`
    Flags                   int     `json:"flags"` // create constant later
    PublicFlags             int     `json:"public_flags"` // create constant later
    Banner                  string  `json:"banner"`
    BannerColor             string  `json:"banner_color"`
    Avatar                  string  `json:"avatar"`
    AvatarDecorationData    string  `json:"avatar_decoration_data"`
}
// user manager
type userManager struct { users map[string]*User }
func (manager *userManager) Get(user_id string) *User {
    return manager.users[user_id]
}
