package discord

// session struct
type session struct {
  Type        string
  Id          string
  Sequence    int
  ApiVersion  int
  Timestamp   int
  Gateway     Connection
  Data        sessionData
  Shards      int
}
type sessionData struct {
  OS            string
  Application   any
  Token         string
  Intents       int
  IntentsArray  []Intent
  Partials      int
}

