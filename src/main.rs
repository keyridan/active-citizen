extern crate telegram_bot;
extern crate rustc_serialize;

mod message;
mod parsing;

//use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use telegram_bot::*;
use parsing::*;
use message::*;

fn main() {
    // Create bot, test simple API call and print bot information
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());
    let res = listen(api);
    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}

fn listen(api: Api) -> Result<()> {
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
        if let Some(message) = u.message {
            let message = check_message(message);
            if let Some(message) = message {
                try!(api.send_message(
                    message.chat_id,
                    format!("Hi, {}! {}", message.name, message.msg),
                    None, None, None, None));
            }
        }
        Ok(ListeningAction::Continue)
    });
    return res;
}

fn check_message(message: Message) -> (Option<ResponseMessage>) {
    match message.msg {
        MessageType::Text(ref text) => {
            let response_message = RequestMessage::new(message.clone(), text.clone());
            println!("{}", response_message);

            let text_type = text.parse().unwrap();
            match text_type {
                TextType::Exit => {
                    return None;
                },
                TextType::Help => {
                    return Some(ResponseMessage::new(response_message.clone(), TextType::Help));
                },
                TextType::Text(text) => {
                    return Some(ResponseMessage::new(response_message.clone(), TextType::Text(text)));
                }
            }
        },
        _ => {
            return None;
        }
    }
}
