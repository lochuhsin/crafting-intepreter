#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {}

pub struct Token {
    token_type: TokenType,
    line: usize,
    lexeme: String,
}

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
