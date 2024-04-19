use crate::manager;
use crate::channel;
use crate::member;

#[derive(Debug)]
pub struct Guild<'a> {
    pub id: String,
    pub channels: manager::Manager<'a, channel::Channel>, 
    pub members: manager::Manager<'a, member::Member<'a>>
}