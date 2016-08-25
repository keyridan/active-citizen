extern crate telegram_bot;

use telegram_bot::*;
use parsing::*;

#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub chat_id: Integer,
    pub name: String,
    pub msg: String,
}

impl From<Message> for ResponseMessage {
    fn from(message: Message) -> ResponseMessage {
        ResponseMessage {
            chat_id: message.chat.id(),
            msg: create_answer_msg(message.msg),
            name: message.from.first_name,
        }
    }
}

fn create_answer_msg(msg: MessageType) -> String {
    let text_typed_message = msg.parse().unwrap();
    match text_typed_message {
        TextType::Help => {
            return format!("We all need help");
        },
        TextType::Text(text) => {
            return format!("You just wrote '{}'", text);
        },
        _ => {
            return format!("(>.<)");
        }
    }
}

trait Parse {
    fn parse(&self) -> Option<TextType>;
}

impl Parse for MessageType {
    fn parse(&self) -> Option<TextType> {
        match *self {
            MessageType::Text(ref text) => Some(text.parse().unwrap()),
            _ => None
        }
    }
}