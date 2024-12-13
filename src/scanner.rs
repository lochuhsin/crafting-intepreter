use crate::errors::error;
use crate::tokens::{Token, TokenType};
#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            String::new(),
            self.line,
        ));
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        let token: Option<TokenType> = match ch {
            // single character tokens
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            // double character tokens
            '!' => {
                if self.match_sub_ch('=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_sub_ch('=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_sub_ch('=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '>' => {
                if self.match_sub_ch('=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            // Comment
            '/' => {
                if self.match_sub_ch('/') {
                    while self.peek() != '\0' && !self.is_at_end() {
                        self.advance();
                    }
                    Some(TokenType::ParserIgnore)
                } else if self.match_sub_ch('*') {
                    let mut closed = false;
                    while !self.is_at_end() {
                        let ch = self.advance();
                        // Note: We need to consume the '/' if we match
                        // not using peek. Otherwise we will get an extra slash token
                        if ch == '*' && self.match_sub_ch('/') {
                            closed = true;
                            break;
                        }

                        if ch == '\n' {
                            self.line += 1;
                        }
                    }
                    if !closed {
                        self.has_error = true;
                        error(self.line, "unclosed block comment".to_string());
                    }
                    Some(TokenType::ParserIgnore)
                } else {
                    Some(TokenType::Slash)
                }
            }
            // Whitespace
            ' ' | '\r' | '\t' => Some(TokenType::ParserIgnore),
            '\n' => {
                self.line += 1;
                Some(TokenType::ParserIgnore)
            }
            // String Literals
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                if self.is_at_end() {
                    self.has_error = true;
                    error(self.line, String::from("Unterminated string literal"));
                    Some(TokenType::ParserIgnore)
                } else {
                    self.advance(); // closing string
                    Some(TokenType::String)
                }
            }
            _ => {
                // put somewhere else
                if ch.is_ascii_digit() {
                    Some(self.match_number())
                } else if ch.is_alphabetic() || ch == '_' {
                    // Not sure why we need _
                    Some(self.match_identifier())
                } else {
                    None
                }
            }
        };
        if let Some(t) = token {
            if t == TokenType::String {
                // we trim the string concatenation ( " )
                self.add_token_with_bound(t, self.start + 1, self.current - 1);
            } else if t == TokenType::Number {
                self.add_token_with_bound(t, self.start, self.current);
            } else if t == TokenType::ParserIgnore {
                //pass
            } else {
                self.add_token(t);
            }
        } else {
            self.has_error = true;
            error(
                self.line,
                format!(
                    "Unexpected character: {}, at {} to {}",
                    &self.source[self.start..self.current],
                    self.start,
                    self.current
                ),
            );
        }
    }

    fn match_identifier(&mut self) -> TokenType {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        if let Some(t) = TokenType::to_keyword(&self.source[self.start..self.current]) {
            t
        } else {
            TokenType::Identifier
        }
    }

    fn match_number(&mut self) -> TokenType {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // fractional, as we don't allow "1234." to be a valid number
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        TokenType::Number
    }

    fn match_sub_ch(&mut self, expect: char) -> bool {
        /*
        if match consumes (advance), else not
        */
        if self.is_at_end() {
            return false;
        }
        if expect != self.source.as_bytes()[self.current] as char {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            String::from(&self.source[self.start..self.current]),
            String::new(),
            self.line,
        ))
    }
    fn add_token_with_bound(&mut self, token_type: TokenType, start: usize, end: usize) {
        self.tokens.push(Token::new(
            token_type,
            String::from(&self.source[start..end]),
            String::new(),
            self.line,
        ))
    }

    fn peek(&self) -> char {
        /*
           Lookahead
        */
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current] as char
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[self.current + 1] as char
        }
    }

    fn advance(&mut self) -> char {
        /*
        I'm Assuming the inputs were always ASCII characters.
        Moreover, the entire logic could be implemented in more rusty way.
        Since there are self.source.chars().nth() or something like that.
        */
        let output = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        output
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
    pub fn get_token_types(&self) -> Vec<TokenType> {
        let mut v: Vec<TokenType> = Vec::new();
        for t in &self.tokens {
            v.push(t.get_token_type());
        }
        v
    }
}
