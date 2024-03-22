package discord

// channel struct
type Channel struct {
  Id        string
  Username  string
}
// channel manager
type channelManager struct { channels map[string]*Channel }
