use crate::manager;
use crate::channel;

#[derive(Debug)]
pub struct Guild<'a> {
    pub id: String,
    pub channels: manager::Manager<'a, channel::Channel> 
}