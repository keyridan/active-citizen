extern crate telegram_bot;
extern crate rustc_serialize;

mod message;
mod parsing;

//use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use telegram_bot::*;
use parsing::*;

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
        // If the received update contains a message...
        if let Some(message) = u.message {
            let message = message::ActiveMessage::new(message);
            println!("{}", message);
            let name = message.from.first_name;

            // Match message type
            match message.msg {
            MessageType::Text(text) => {

                let text_type = text.parse().unwrap();
                match text_type {
                        TextType::Exit => {
                            return Ok(ListeningAction::Stop);
                        },
                        TextType::Help => {
                            try!(api.send_message(
                                message.chat.id(),
                                format!("Hi, {}! I can't help you right now. Use /exit for exit", name),
                                None, None, None, None));
                        },
                        TextType::Text(text) => {
                            try!(api.send_message(
                                message.chat.id(),
                                format!("Hi, {}! You just wrote {} ", name, text),
                                None, None, None, None));
                        }
                    }
                },
                _ => {}
            }
        }
        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });
    return res;
}
