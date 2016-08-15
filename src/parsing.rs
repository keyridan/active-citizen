use std::str::FromStr;

pub enum TextType {
    Exit,
    Help,
    Text(String),
}

impl FromStr for TextType {
    type Err = ();
    fn from_str(s: &str) -> Result<TextType, ()> {
        match s {
            "/exit" => Ok(TextType::Exit),
            "/help" => Ok(TextType::Help),
            _ => Ok(TextType::Text(String::from(s))),
        }
    }
}

#[test]
fn test_parse_exit() {
    let res: Result<TextType, ()> = "/exit".parse();
    assert!(res.is_ok());
}

#[test]
fn test_parse_help() {
    let res: Result<TextType, ()> = "/help".parse();
    assert!(res.is_ok());
}

#[test]
fn test_parse_hi() {
    let res: Result<TextType, ()> = "hi".parse();
    assert!(res.is_ok());
}