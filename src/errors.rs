use crate::tokens::{Token, TokenType};

pub fn error(line: usize, msg: String) {
    report(line, String::new(), msg)
}

fn report(line: usize, wh: String, msg: String) {
    println!("line[ {} ] Error {} : {}", line, wh, msg);
}

pub fn parse_error(token: &Token, msg: &str) {
    if *token.get_token_type() == TokenType::EOF {
        report(token.get_line(), "at end".to_string(), msg.to_string());
    } else {
        report(
            token.get_line(),
            format!("at {}", token.get_lexeme()),
            msg.to_string(),
        )
    }
}
