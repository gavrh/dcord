// dcrs - Discord API Library
pub mod guild;
pub mod channel;
pub mod role;
pub mod emoji;
pub mod invite;
pub mod user;
pub mod member;
pub mod message;
pub mod event;
pub mod gateway;
mod manager;

/// ### Client Struct
/// The **Client** struct is the base of the entire "operation".
#[derive(Debug)]
pub struct Client<'a> {
    pub user: user::ClientUser,
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
    pub fn new(intents: Vec<gateway::GatewayIntent>) -> Client<'a> {
        return Client {
            user: user::ClientUser {
                id: "".into(),
                username: "".into(),
                discriminator: "".into(),
                name: "".into(),
                avatar_hash: "".into(),
                banner_hash: "".into(),
                accent_color: 0,
                verified: false,
                flags: vec!(),
                premium_kind: user::UserPremiumKind::NONE,
                avatar_decoration_hash: "".into()
            },
            guilds: manager::Manager::init(),
            users: manager::Manager::init(),
        }
    }

    pub fn bind_handler<T>(&self, handler: T) {

    }

    pub fn login(&self, token: String) {
        println!("{token}");
    }

}

#[cfg(test)]
mod tests {}