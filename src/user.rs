#[derive(Debug)]
pub struct ClientUser {

}

#[derive(Debug)]
pub struct User {
    id:         String,
    username:   String,
    persona:    String
}

#[derive(Debug)]
pub struct UserManager<'a> {
    pub(crate) users: std::collections::HashMap<String, &'a User>

    
}