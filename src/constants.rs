///! A set of constants used by the library.

pub const EMBED_MAX_LENGTH: usize = 6000;
pub const EMBED_MAX_COUNT: usize = 10;
pub const STICKER_MAX_COUNT: usize = 3;
pub const GATEWAY: &str = "wss://gateway.discord.gg/";
pub const GATEWAY_VERSION: u8 = 10;
pub const LARGE_THRESHOLD: u8 = 250;
pub const MESSAGE_CODE_LIMIT: usize = 2000;
pub const MEMBER_FETCH_LIMIT: usize = 1000;
pub const USER_AGENT: &str = concat!("dcrs v", env!("CARGO_PKG_VERSION"));