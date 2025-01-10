mod chat_client;
mod handler_vec;
mod types;

pub use chat_client::ChatClient;

use crate::channel::ChannelId;
use std::time::SystemTime;

#[derive(Clone)]
pub struct ChatEvent {
    pub time: SystemTime,
    pub message: String,
    pub user: ChannelId,
}
