extern crate telegram_bot;
extern crate rustc_serialize;

mod message;
mod parsing;

use telegram_bot::*;
use message::*;

fn main() {
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
            let generated_response_message = generate_response(message);
            if let Some(response_message) = generated_response_message {
                send_response(response_message, &api);
            }
        }
        Ok(ListeningAction::Continue)
    });
    return res;
}

fn generate_response(message: Message) -> (Option<ResponseMessage>) {
    match message.msg {
        MessageType::Text(_) => {
            let response_message = RequestMessage::from(message.clone());
            println!("{}", response_message);
            return Some(ResponseMessage::from(response_message.clone()));
        },
        _ => {
            return None;
        }
    }
}

fn send_response(message: ResponseMessage, api: &Api) {
    api.send_message(
        message.chat_id,
        format!("Hi, {}! {}", message.name, message.msg),
        None, None, None, None);
}
