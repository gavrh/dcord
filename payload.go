package discord

// receive payload struct
type payload struct {
	Opcode		int		`json:"op"`
	Sequence	int		`json:"s"`
	Type		string 	`json:"t"`
}
// send payload struct
type sendPaylod struct {
	Opcode		int				`json:"op"`
	Sequence	int				`json:"s"`
	Type		string			`json:"t"`
	Data		map[string]any	`json:"d"`
}
// ready payload
type readyData struct {
	User	*ClientUser `json:"user"`
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