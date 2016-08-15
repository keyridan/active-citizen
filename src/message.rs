extern crate telegram_bot;

use telegram_bot::*;
use std::fmt;

pub struct ActiveMessage {
    pub message_id: Integer,
    pub from: User,
    pub chat: Chat,
    pub date: Integer,

    pub forward: Option<(User, Integer)>,
    pub reply: Option<Box<Message>>,

    pub msg: MessageType,

    pub caption: Option<String>,
}

impl ActiveMessage {
    pub fn new(message: Message) -> ActiveMessage {
        ActiveMessage {
            from: (message.from),
            chat: (message.chat),
            date: (message.date),
            forward: (message.forward),
            reply: (message.reply),
            msg: (message.msg),
            caption: (message.caption),
            message_id: (message.message_id),
        }
    }
}

impl fmt::Display for ActiveMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = &self.from.first_name;
        let msg = &self.msg;
        let message_id = &self.message_id;
        let chat_id = &self.chat.id();
        write!(f, "Message: from={}, msg={:?}, msgId={}, chatId={}", name, msg, message_id, chat_id)
    }
}