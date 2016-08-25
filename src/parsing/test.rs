use parsing::*;

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