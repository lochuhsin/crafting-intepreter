use crate::chunk::Chunk;
use crate::errors::error;
use crate::rules::{ParseFn, ParseRule, Precedence};
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};
use crate::values::GenericValue;
use crate::vm::disassemble_chunk;
use crate::vm::OpCode;

/*
 * TODO: Add ternary operator support
 */

pub fn compile(s: String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(s);
    let mut parser = Parser::new();
    parser.advance(&mut scanner);
    expression(&mut parser, &mut scanner, chunk);
    parser.consume(TokenType::EOF, &mut scanner, "Expect end of expression");
    end_compiler(chunk, parser.previous.unwrap().get_line(), parser.had_error);
    !parser.had_error
}
#[derive(Default)]
pub struct Parser {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            let token = scanner.scan_token();
            let token_type = *token.get_token_type();
            self.current = Some(token.clone()); // this is slow
            if token_type != TokenType::ParseError {
                break;
            }
            self.panic_mode = true;
            error_at(&token, &token.get_lexeme());
        }
    }
    pub fn consume(&mut self, token_type: TokenType, scanner: &mut Scanner, msg: &str) {
        if let Some(token) = self.current.clone() {
            if token.get_token_type() == &token_type {
                self.advance(scanner);
            } else {
                error_at(&token, msg);
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

fn number(previous_token: Option<Token>, chunk: &mut Chunk) {
    let token: &Token = previous_token.as_ref().unwrap();
    let num = token.get_lexeme().parse::<f64>().unwrap();
    let value = GenericValue::from_number(num);
    emit_constant(token.get_line(), value, chunk);
}

fn binary(
    parser: &mut Parser,
    scanner: &mut Scanner,
    previous_token: Option<Token>,
    chunk: &mut Chunk,
) {
    let cloned_previous = previous_token.clone();
    let token = previous_token.as_ref().unwrap();
    let op = token.get_token_type();
    let rule = ParseRule::get_rule(*op).unwrap();
    parse_precedence(
        parser,
        cloned_previous,
        scanner,
        Precedence::from_usize(rule.precedence as usize + 1),
        chunk,
    );
    let line = token.get_line();
    match op {
        TokenType::Plus => emit_byte(chunk, OpCode::OpAdd as usize, line),
        TokenType::Minus => emit_byte(chunk, OpCode::OpSubtract as usize, line),
        TokenType::Star => emit_byte(chunk, OpCode::OpMultiply as usize, line),
        TokenType::Slash => emit_byte(chunk, OpCode::OpDivide as usize, line),
        TokenType::EqualEqual => emit_byte(chunk, OpCode::OpEqual as usize, line),

        // Implement the below >=, <=, != using one opcode, since it is faster
        TokenType::BangEqual => emit_bytes(
            chunk,
            OpCode::OpEqual as usize,
            OpCode::OpNot as usize,
            line,
        ),
        TokenType::Greater => emit_byte(chunk, OpCode::OpGreater as usize, line),
        TokenType::GreaterEqual => {
            emit_bytes(chunk, OpCode::OpLess as usize, OpCode::OpNot as usize, line)
        }
        TokenType::Less => emit_byte(chunk, OpCode::OpLess as usize, line),
        TokenType::LessEqual => emit_bytes(
            chunk,
            OpCode::OpGreater as usize,
            OpCode::OpNot as usize,
            line,
        ),
        _ => (), // unreachable
    }
}

fn unary(
    parser: &mut Parser,
    scanner: &mut Scanner,
    previous_token: Option<Token>,
    chunk: &mut Chunk,
) {
    let cloned = previous_token.clone();
    let token = previous_token.as_ref().unwrap();
    let op = token.get_token_type();

    parse_precedence(parser, cloned, scanner, Precedence::PrecUnary, chunk);
    // self.parse_precedence(Precedence::PrecUnary);
    // Compile the operand
    expression(parser, scanner, chunk);

    match op {
        TokenType::Minus => {
            emit_byte(chunk, OpCode::OpNegate as usize, token.get_line());
        }
        TokenType::Bang => {
            emit_byte(chunk, OpCode::OpNot as usize, token.get_line());
        }
        _ => (), // will add a lot
    }
}

fn literal(previous_token: Option<Token>, chunk: &mut Chunk) {
    let token = previous_token.as_ref().unwrap();
    match *token.get_token_type() {
        TokenType::False => emit_byte(chunk, OpCode::OpFalse as usize, token.get_line()),
        TokenType::Nil => emit_byte(chunk, OpCode::OpNil as usize, token.get_line()),
        TokenType::True => emit_byte(chunk, OpCode::OpTrue as usize, token.get_line()),
        _ => (), // unreachable
    }
}

fn grouping(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    expression(parser, scanner, chunk);
    parser.consume(
        TokenType::RightParen,
        scanner,
        "Expect ')' after expression",
    );
}

fn expression(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    let previous_token = parser.previous.clone();
    parse_precedence(
        parser,
        previous_token,
        scanner,
        Precedence::PrecAssignment,
        chunk,
    );
}

fn parse_precedence(
    parser: &mut Parser,
    previous_token: Option<Token>,
    scanner: &mut Scanner,
    precedence: Precedence,
    chunk: &mut Chunk,
) {
    parser.advance(scanner);

    // NOTE: Handle this parser if previous is None
    let token = previous_token.as_ref().unwrap();
    let previous_type = token.get_token_type();

    let rule = ParseRule::get_rule(*previous_type).unwrap();
    let prefix_rule = rule.prefix;
    if prefix_rule == ParseFn::Null {
        error(token.get_line(), "Expect expression")
    }
    // this is prefixRule() in the book, since I'm not sure how to store function pointers at this moment
    execute_parsfn(parser, prefix_rule, scanner, chunk);

    loop {
        let curr_token = parser.current.as_mut().unwrap();
        let rule = ParseRule::get_rule(*curr_token.get_token_type()).unwrap();
        if precedence as usize <= rule.precedence as usize {
            parser.advance(scanner);
            let infix_rule = ParseRule::get_rule(*previous_type).unwrap().infix;
            execute_parsfn(parser, infix_rule, scanner, chunk);
        } else {
            break;
        }
    }
}

fn execute_parsfn(parser: &mut Parser, parsfn: ParseFn, scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = parser.previous.clone();
    match parsfn {
        ParseFn::Literal => literal(token, chunk),
        ParseFn::Number => number(token, chunk),
        ParseFn::Unary => unary(parser, scanner, token, chunk),
        ParseFn::Binary => binary(parser, scanner, token, chunk),
        ParseFn::Grouping => grouping(parser, scanner, chunk),
        ParseFn::Null => (),
    }
}

fn emit_byte(chunk: &mut Chunk, byte: usize, previous_line: usize) {
    chunk.write_chunk(byte, previous_line);
}

fn emit_bytes(chunk: &mut Chunk, byte1: usize, byte2: usize, previous_line: usize) {
    emit_byte(chunk, byte1, previous_line);
    emit_byte(chunk, byte2, previous_line);
}

fn end_compiler(chunk: &mut Chunk, previous_line: usize, has_error: bool) {
    #[cfg(debug_assertions)]
    {
        disassemble_chunk(chunk, "code");
    }
    emit_byte(chunk, OpCode::OpReturn as usize, previous_line);
}

fn emit_constant(previous_line: usize, value: GenericValue, chunk: &mut Chunk) {
    let cont_operl = make_constant(value, chunk);
    emit_bytes(
        chunk,
        OpCode::OpConstant as usize,
        cont_operl,
        previous_line,
    );
}

fn make_constant(value: GenericValue, chunk: &mut Chunk) -> usize {
    chunk.add_const(value)
}
