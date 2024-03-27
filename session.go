package discord

// session struct
type session struct {
    Type        string
    Id          string
    Sequence    int
    Timestamp   int
    Gateway     Connection
    Data        sessionData
    Done        chan bool
    Shards      int
}
type sessionData struct {
    OS            string
    ApiVersion    int 
    Application   any
    Token         string
    Intents       int
    IntentsArray  []Intent
    Partials      int
    Debug         bool
    AllReady      bool
}

