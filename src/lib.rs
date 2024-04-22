// dcrs - Discord API Library
pub mod guild;
pub mod channel;
pub mod role;
pub mod emoji;
pub mod invite;
pub mod user;
pub mod member;
pub mod message;
mod manager;

/// ### Client Struct
/// The **Client** struct is the base of the entire "operation".
#[derive(Debug)]
pub struct Client<'a> {
    pub guilds: manager::Manager<'a, guild::Guild<'a>>,
    pub users:  manager::Manager<'a, user::User>
}

impl<'a> Client<'a> {

    /// ### Create New Client
    /// 
    /// ```rs
    /// // example
    /// let client = dcrs::Client::new();
    /// ```
    pub fn new() -> Client<'a> {
        return Client {
            guilds: manager::Manager::init(),
            users: manager::Manager::init()
        }
    }

    /// #### Client.login(token)
    pub fn login(token: String) {
        print!("{token}\n");
    }

}

/// ### Gateway Intents
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Intent {
    GUILDS                          = 1<<0,
    GUILD_MEMBERS                   = 1<<1,
    GUILD_MODERATION                = 1<<2,
    GUILD_EMOJIS_AND_STICKERS       = 1<<3,
    GUILD_INTEGRATIONS              = 1<<4,
    GUILD_WEBHOOKS                  = 1<<5,
    GUILD_INVITES                   = 1<<6,
    GUILD_VOICE_STATES              = 1<<7,
    GUILD_PRESENCES                 = 1<<8,
    GUILD_MESSAGES                  = 1<<9,
    GUILD_MESSAGE_REACTIONS         = 1<<10,
    GUILD_MESSAGE_TYPING            = 1<<11,
    DIRECT_MESSAGES                 = 1<<12,
    DIRECT_MESSAGE_REACTIONS        = 1<<13,
    DIRECT_MESSAGE_TYPING           = 1<<14,
    MESSAGE_CONTENT                 = 1<<15,
    GUILD_SCHEDULED_EVENTS          = 1<<16,
    AUTO_MODERATION_CONFIGURATION   = 1<<20,
    AUTO_MODERATION_EXECUTION       = 1<<21
}

#[cfg(test)]
mod tests {}