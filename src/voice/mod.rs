use crate::utils::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum VoiceOpcode {
    Identify,
    SelectProtocol,
    Ready,
    Heartbeat,
    SessionDescription,
    Speaking,
    HeartbeatAck,
    Resume,
    Hello,
    Resumed,
    ClientDisconnect = 13
}

pub mod close_codes {
    /// Invalid Voice OP Code.
    pub const INVALID_OPCODE: u16 = 4001;
    /// An invalid payload was sent.
    pub const DECODE_ERROR: u16 = 4002;
    /// A payload was sent prior to identifying.
    pub const NOT_AUTHENTICATED: u16 = 4003;
    /// The account token sent with the identify payload was incorrect.
    pub const AUTHENTICATION_FAILED: u16 = 4004;
    /// More than one identify payload was sent.
    pub const ALREADY_AUTHENTICATED: u16 = 4005;
    /// The session is no longer valid.
    pub const INVALID_SESSION: u16 = 4006;
    /// The session timed out.
    pub const SESSION_TIMEOUT: u16 = 4009;
    /// Server attempted to connect to is unable to be found.
    pub const INVALID_SERVER: u16 = 4011;
    /// Unable to recognize protocol that was sent.
    pub const UNKNOWN_PROTOCOL: u16 = 4012;
    /// Channel was deleted, you were kicked, voice server changed,
    /// or the main gateway session was dropped.
    /// 
    /// Should not reconnect.
    pub const DISCONNECTED: u16 = 4014;
    /// The server crashed. Try resuming.
    pub const SERVER_CRASHED: u16 = 4015;
    /// Unable to recognize encryption that was sent.
    pub const UNKNOWN_ENCRYPTION: u16 = 4016; 
}