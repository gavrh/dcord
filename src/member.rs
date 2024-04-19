use crate::user;

#[derive(Debug)]
pub struct Member<'a> {
    user: &'a user::User
}