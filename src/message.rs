extern crate telegram_bot;

use telegram_bot::*;
use std::fmt;
use parsing::*;

#[derive(Debug, Clone)]
pub struct RequestMessage {
    pub message_id: Integer,
    pub from: User,
    pub chat: Chat,
    pub date: Integer,

    pub forward: Option<(User, Integer)>,
    pub reply: Option<Box<Message>>,

    pub msg: String,

    pub caption: Option<String>,
}

impl RequestMessage {
    pub fn new(message: Message, msg: String) -> RequestMessage {
        RequestMessage {
            from: (message.from),
            chat: (message.chat),
            date: (message.date),
            forward: (message.forward),
            reply: (message.reply),
            msg: (msg),
            caption: (message.caption),
            message_id: (message.message_id),
        }
    }
}

impl fmt::Display for RequestMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = &self.from.first_name;
        let msg = &self.msg;
        let message_id = &self.message_id;
        let chat_id = &self.chat.id();
        write!(f, "Message: from={}, msg=\"{}\", msgId={}, chatId={}", name, msg, message_id, chat_id)
    }
}

#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub chat_id: Integer,
    pub name: String,
    pub msg: String,
}

impl ResponseMessage {
    pub fn new(message: RequestMessage, msg: TextType) -> ResponseMessage {
        ResponseMessage {
            chat_id: (message.chat.id()),
            msg: (create_answer_msg(msg)),
            name: (message.from.first_name)
        }
    }
}

fn create_answer_msg(msg: TextType) -> String {
    match msg {
        TextType::Help => {
            return format!("We all need help");
        },
        TextType::Text(text) => {
            return text;
        },
        _ => {
            return format!("(>.<)");
        }
    }
}