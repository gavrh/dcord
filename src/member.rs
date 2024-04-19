use crate::user;

#[derive(Debug)]
pub struct Member<'a> {
    pub user: &'a user::User
}