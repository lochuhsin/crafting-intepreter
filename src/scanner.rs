use crate::token::Token;

pub struct Scanner {
    content: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(content: String) -> Scanner {
        Scanner {
            content,
            start: 0,
            current: 0,
            line: 0,
        }
    }
}

fn scan_token() -> Token {
    unimplemented!()
}
