use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
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

impl fmt::Display for TextType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_value = match *self {
            TextType::Exit => "/exit",
            TextType::Help => "/help",
            TextType::Text(ref text) => &text,
        };
        write!(f, "TextType: {}", type_value)
    }
}

// ===========================================================================
// Unit tests
// ===========================================================================
mod test;