use crate::guild;
use crate::channel;
use crate::user;

#[derive(Debug)]
pub struct Invite<'a> {
    pub code:               String,
    pub guild:              &'a guild::Guild<'a>,
    pub channel:            &'a channel::Channel<'a>,
    pub inviter:            &'a user::User,
    pub target_kind:        InviteTargetKind, 
    pub target_user:        &'a user::User,
    pub target_application: Option<String>, // application type
    // presence count
    // member count
    // expires at
    // stag instance
    // guild scheduled events
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum InviteTargetKind {
    STREAM                  = 1,
    EMBEDDED_APPLICATION    = 2
}