// dcrs - Discord API Library
pub mod guild;
pub mod channel;
pub mod user;
pub mod manager;

#[derive(Debug)]
pub struct Client<'a> {
    pub guilds: manager::Manager<'a, guild::Guild<'a>>,
    pub users: manager::Manager<'a, user::User>
}

impl<'a> Client<'a> {

    pub fn new() -> Client<'a> {
        return Client {
            guilds: manager::Manager::init(),
            users: manager::Manager::init()
        }
    }

}

#[cfg(test)]
mod tests {}