use crate::role;
use crate::user;

#[derive(Debug)]
pub struct Emoji<'a> {
    pub id:             String,
    pub name:           String,
    pub roles:          Vec<&'a role::Role<'a>>,
    pub user:           &'a user::User,
    pub require_colons: bool,
    pub managed:        bool,
    pub animated:       bool,
    pub available:      bool
}