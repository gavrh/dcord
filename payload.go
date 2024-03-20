package discord

// receive payload struct
type payload struct {
	Opcode		int		`json:"op"`
	Sequence	int		`json:"s"`
	Type		Event 	`json:"t"`
}
// send payload struct
type sendPaylod struct {
	Opcode		int				`json:"op"`
	Sequence	int				`json:"s"`
	Type		Event			`json:"t"`
	Data		map[string]any	`json:"d"`
}
// ready payload
type readyData struct {
	GatewayVersion 			int					`json:"v"`
	UserSettings 			map[string]any		`json:"user_settings"`
	User					*ClientUser 		`json:"user"`
	SessionType 			string				`json:"session_type"`
	SessionId				string				`json:"session_id"`
	ResumeGatewayUrl 		string				`json:"resume_gateway_url"`
	Realtionships 			[]any				`json:"relationships"`
	PrivateChannels 		[]any				`json:"private_channels"`
	Presences				[]any				`json:"presences"`
	Guilds 					[]map[string]any	`json:"guils"`
	GuildJoinRequests		[]any				`json:"guild_join_requests"`
	GeoOrderedRtcRegions	[]string			`json:"geo_ordered_rtc_regions"`
	Auth					map[string]any		`json:"auth"`
	Application				map[string]any		`json:"application"`
}
type readyPayload struct {
	Data readyData `json:"d"`
}
// message create payload
type messageCreateData struct {

}
type messageCreatePayload struct {
	Data messageCreateData `json:"d"`
}