use crate::chunk;
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::rules::{ParseFn, ParseRule, Precedence};
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};
use crate::value::Value;

pub fn compile(s: String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(s);
    // scanner.scan_token();
    let tokens = scanner.get_tokens();
    let mut parser = Parser::new(tokens.clone(), chunk.clone());
    parser.advance();
    expression(&mut parser, chunk);
    parser.consume(TokenType::EOF, "expect end of expression");
    // end_compiler();
    true
}

pub struct Parser {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    current_i: usize,
    tokens: Vec<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, chunk: Chunk) -> Parser {
        Parser {
            current: None,
            previous: None,
            current_i: 0,
            tokens,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();
        while !self.is_at_end() {
            let token = self.scan_token();
            if token.get_token_type() != &TokenType::ParseError {
                self.current = Some(token);
                break;
            }
            self.had_error = true;
            self.panic_mode = true;
            error_at(&token, &token.get_lexeme());
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_i == self.tokens.len()
    }

    fn scan_token(&mut self) -> Token {
        let token = self.tokens[self.current_i].clone();
        self.current_i += 1;
        token
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &str) -> bool {
        if let Some(token) = self.current.clone() {
            if token.get_token_type() == &token_type {
                self.advance();
                true
            } else {
                error_at(&token, msg);
                false
            }
        } else {
            panic!("self.current is None .... figure out why")
        }
    }
}

fn error_at(token: &Token, msg: &str) {
    print!("[line {}] Error", token.get_line());
    if token.get_token_type() == &TokenType::EOF {
        print!(" at end")
    } else if token.get_token_type() == &TokenType::ParseError {
    } else {
        print!(" at {}", token.get_lexeme())
    }
    println!(": {}", msg);
}

fn number(parser: &mut Parser, previous_token: Option<Token>, chunk: &mut Chunk) {
    let token = previous_token.as_ref().unwrap();
    let value: f64 = token.get_lexeme().parse::<Value>().unwrap();

    emit_constant(token, value, chunk);
}

fn unary(parser: &mut Parser, previous_token: Option<Token>, chunk: &mut Chunk) {
    let token = previous_token.as_ref().unwrap();
    let op = token.get_token_type();
    parse_precedence(parser, Precedence::PrecUnary, chunk);
    // self.parse_precedence(Precedence::PrecUnary);
    // Compile the operand
    expression(parser, chunk);

    match op {
        TokenType::Minus => {
            emit_byte(token, OpCode::OpNegate as usize, chunk);
        }
        _ => (),
    }
}

fn binary(parser: &mut Parser, previous_token: Option<Token>, chunk: &mut Chunk) {
    let token = previous_token.as_ref().unwrap();
    let op = token.get_token_type();
    let rule = ParseRule::get_rule(*op).unwrap();
    parse_precedence(
        parser,
        Precedence::from_usize(rule.precedence as usize + 1),
        chunk,
    );
    match op {
        TokenType::Plus => emit_byte(token, OpCode::OpAdd as usize, chunk),
        TokenType::Minus => emit_byte(token, OpCode::OpSubtract as usize, chunk),
        TokenType::Star => emit_byte(token, OpCode::OpMultiply as usize, chunk),
        TokenType::Slash => emit_byte(token, OpCode::OpDivide as usize, chunk),
        _ => (), // unreachable
    }
}

fn grouping(parser: &mut Parser, chunk: &mut Chunk) {
    expression(parser, chunk);
    parser.consume(TokenType::RightParen, "Expect ')' after expression");
}

fn expression(parser: &mut Parser, chunk: &mut Chunk) {
    parse_precedence(parser, Precedence::PrecAssignment, chunk);
}

fn parse_precedence(parser: &mut Parser, precedence: Precedence, chunk: &mut Chunk) {
    parser.advance();
    let previous_type = parser.previous.as_ref().unwrap().get_token_type();

    if let Some(rule) = ParseRule::get_rule(*previous_type) {
        let prefix_rule = rule.prefix;
        execute_parsfn(parser, prefix_rule, chunk);
    } else {
        error_at(
            &parser.previous.as_ref().unwrap().clone(),
            "Expect expression",
        );
    }
}

fn execute_parsfn(parser: &mut Parser, parsfn: ParseFn, chunk: &mut Chunk) {
    let token = parser.previous.clone();
    match parsfn {
        ParseFn::Number => number(parser, token, chunk),
        ParseFn::Unary => unary(parser, token, chunk),
        ParseFn::Binary => binary(parser, token, chunk),
        ParseFn::Grouping => grouping(parser, chunk),
        ParseFn::Null => (),
    }
}

fn emit_byte(token: &Token, byte: usize, chunk: &mut Chunk) {
    chunk.write_chunk(byte, token.get_line());
}

fn emit_bytes(token: &Token, byte1: usize, byte2: usize, chunk: &mut Chunk) {
    emit_byte(token, byte1, chunk);
    emit_byte(token, byte2, chunk);
}

fn end_compiler(token: &Token, chunk: &mut Chunk) {
    emit_byte(token, OpCode::OpReturn as usize, chunk);
}

fn emit_constant(token: &Token, value: Value, chunk: &mut Chunk) {
    let cont_operl = make_constant(value, chunk);
    emit_bytes(token, OpCode::OpConstant as usize, cont_operl, chunk);
}

fn make_constant(value: Value, chunk: &mut Chunk) -> usize {
    chunk.add_const(value)
}
