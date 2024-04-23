use crate::utils::*;

#[derive(Serialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum WsSendData<'a> {
    Heartbeat(Option<u64>),
    Identify {
        compress: bool,
        token: &'a str,
        large_threshold: u8,
        shard: &'a str,
        intents: u64,
    },
    PresenceUpdate,
    VoiceStateUpdate,
    Resume {
        session_id: &'a str,
        token: &'a str,
        seq: u64
    },
    RequestGuildMembers
}

#[derive(Serialize)]
struct WsWritePayload<'a> {
    op: super::GatewayOpcode,
    d:  WsSendData<'a>
}

#[derive(Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum WsReceiveData {
    Dipatch,
    Heartbeat,
    Reconnect,
    InvalidSession,
    Hello,
    HearbeatAck
}