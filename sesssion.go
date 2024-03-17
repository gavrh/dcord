package discord

// # Session Data Struct
type SessionData struct {
	OS             string
	Application    any
	Token          string
	Intents        int
	Partials       any
	EventCallbacks map[string]any
}

// # Session Struct
type Session struct {
	Type           string
	Id             string
	Sequence       int
	ApiVersion     int
	GatewayVersion int
	Timestamp      int
	Gateway        Connection
	Data           SessionData
	Shards         int
}
