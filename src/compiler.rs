use crate::chunk::Chunk;
use crate::errors::error;
use crate::parser::Parser;
use crate::rules::{ParseFn, ParseRule, Precedence};
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};
use crate::values::GenericValue;
use crate::vm::disassemble_chunk;
use crate::vm::OpCode;

/*
 *
 *
 * declaration -> classDecl | funcDecl | varDecl | statement;
 *
 * statement   -> exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block
 *
 * TODO: Add ternary operator support
 */

pub fn compile(s: String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(s);
    let mut parser = Parser::new();
    parser.advance(&mut scanner); // Not sure why do we need this, instead of initialize previous as None, and current is the first token ..., maybe there are reasons in the book
    while !match_token(&mut parser, &mut scanner, TokenType::EOF) {
        declaration(&mut parser, &mut scanner, chunk);
    }
    end_compiler(chunk, parser.previous.unwrap().get_line());
    !parser.had_error
}

fn match_token(parser: &mut Parser, scanner: &mut Scanner, token_type: TokenType) -> bool {
    if !check(&token_type, parser.current.as_ref().unwrap().get_type()) {
        false
    } else {
        parser.advance(scanner);
        true
    }
}

fn check(expect_token_type: &TokenType, current_token_type: &TokenType) -> bool {
    expect_token_type == current_token_type
}

pub fn declaration(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    if match_token(parser, scanner, TokenType::Var) {
        var_declaration(parser, scanner, chunk)
    } else {
        statement(parser, scanner, chunk);
    }
}

fn var_declaration(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    let global_var = parse_variable(parser, scanner, chunk, "Expect variable name");
    if match_token(parser, scanner, TokenType::Equal) {
        expression(parser, scanner, chunk);
    } else {
        emit_byte(
            chunk,
            OpCode::OpNil as usize,
            parser
                .previous
                .as_ref()
                .expect("previous token in val declaration should not be none")
                .get_line(),
        );
    }
    parser.consume(
        TokenType::Semicolon,
        scanner,
        "Expect ';' after variable declaration",
    );
    define_variable(global_var, parser, chunk);
}

fn parse_variable(
    parser: &mut Parser,
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    msg: &str,
) -> usize {
    parser.consume(TokenType::Identifier, scanner, msg);
    identifier_constant(parser.previous.as_ref(), chunk)
}

fn identifier_constant(previous_token: Option<&Token>, chunk: &mut Chunk) -> usize {
    let lexeme = previous_token.unwrap().get_lexeme();
    make_constant(GenericValue::from_string(lexeme), chunk)
}

fn define_variable(global_var: usize, parser: &mut Parser, chunk: &mut Chunk) {
    emit_bytes(
        chunk,
        OpCode::OpDefineGlobal as usize,
        global_var,
        parser
            .previous
            .as_ref()
            .expect("[define variable] previous token should exists")
            .get_line(),
    );
}

fn statement(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    if match_token(parser, scanner, TokenType::Print) {
        print_statement(parser, scanner, chunk);
    } else {
        expression_statement(parser, scanner, chunk)
    }
}

fn expression_statement(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    expression(parser, scanner, chunk);
    parser.consume(TokenType::Semicolon, scanner, "Expect ';' after expression");
    emit_byte(
        chunk,
        OpCode::OpPop as usize,
        parser.previous.as_ref().unwrap().get_line(),
    );
}

fn print_statement(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    expression(parser, scanner, chunk);

    parser.consume(TokenType::Semicolon, scanner, "Expect ';' after value");
    emit_byte(
        chunk,
        OpCode::OpPrint as usize,
        parser.previous.as_ref().unwrap().get_line(),
    );
}

fn variable(previous_token: Option<Token>, chunk: &mut Chunk) {
    named_variable(previous_token.as_ref(), chunk);
}

fn named_variable(previous_token: Option<&Token>, chunk: &mut Chunk) {
    let arg = identifier_constant(previous_token, chunk);
    emit_bytes(
        chunk,
        OpCode::OpGetGlobal as usize,
        arg,
        previous_token
            .expect("name variable token should not be empty")
            .get_line(),
    );
}

fn string(previous_token: Option<Token>, chunk: &mut Chunk) {
    let token = previous_token.as_ref().unwrap();
    emit_constant(
        token.get_line(),
        GenericValue::from_string(token.get_lexeme()),
        chunk,
    );
}

fn number(previous_token: Option<Token>, chunk: &mut Chunk) {
    let token: &Token = previous_token.as_ref().unwrap();
    let num = token
        .get_lexeme()
        .parse::<f64>()
        .expect("if a token gets in to this number state, it must be f64, fix the error");
    emit_constant(token.get_line(), GenericValue::from_f64(num), chunk);
}

fn binary(
    parser: &mut Parser,
    scanner: &mut Scanner,
    previous_token: Option<Token>,
    chunk: &mut Chunk,
) {
    let token = previous_token
        .as_ref()
        .expect("<Binary>, there should be no exceptions while getting token ");
    let op = token.get_type();
    let rule = ParseRule::get_rule(*op);
    parse_precedence(
        parser,
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
        TokenType::GreaterEqual => emit_byte(chunk, OpCode::OpGreaterEqual as usize, line),
        TokenType::Less => emit_byte(chunk, OpCode::OpLess as usize, line),
        TokenType::LessEqual => emit_byte(chunk, OpCode::OpLessEqual as usize, line),
        _ => (), // unreachable
    }
}

fn unary(
    parser: &mut Parser,
    scanner: &mut Scanner,
    previous_token: Option<Token>,
    chunk: &mut Chunk,
) {
    let token = previous_token.as_ref().unwrap();
    let op = token.get_type();

    parse_precedence(parser, scanner, Precedence::PrecUnary, chunk);
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
    match *token.get_type() {
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

pub fn expression(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    parse_precedence(parser, scanner, Precedence::PrecAssignment, chunk);
}

fn parse_precedence(
    parser: &mut Parser,
    scanner: &mut Scanner,
    precedence: Precedence,
    chunk: &mut Chunk,
) {
    parser.advance(scanner);
    let token = parser
        .previous
        .clone()
        .expect("There should be no exceptions while getting the previous token");
    // NOTE: Handle this parser if previous is None
    let previous_type: &TokenType = token.get_type();
    let rule = ParseRule::get_rule(*previous_type);

    if rule.prefix == ParseFn::Null {
        error(token.get_line(), "Expect expression")
    } else {
        // this is prefixRule() in the book, since I'm not sure how to store function pointers at this moment
        execute_parsfn(parser, rule.prefix, scanner, chunk);
    }

    loop {
        let curr_token = parser.current.as_mut().unwrap();
        let rule = ParseRule::get_rule(*curr_token.get_type());
        if precedence as usize <= rule.precedence as usize {
            parser.advance(scanner);

            let infix_rule = ParseRule::get_rule(
                *parser
                    .previous
                    .as_ref()
                    .expect("previous token in execute_parsfn should not be None")
                    .get_type(),
            )
            .infix;
            execute_parsfn(parser, infix_rule, scanner, chunk);
        } else {
            break;
        }
    }
}

fn execute_parsfn(parser: &mut Parser, parsfn: ParseFn, scanner: &mut Scanner, chunk: &mut Chunk) {
    let token: Option<Token> = parser.previous.clone(); // don't like this
    match parsfn {
        ParseFn::Literal => literal(token, chunk),
        ParseFn::Number => number(token, chunk),
        ParseFn::Unary => unary(parser, scanner, token, chunk),
        ParseFn::Binary => binary(parser, scanner, token, chunk),
        ParseFn::Grouping => grouping(parser, scanner, chunk),
        ParseFn::String => string(token, chunk),
        ParseFn::Variable => variable(token, chunk),
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

fn end_compiler(chunk: &mut Chunk, previous_line: usize) {
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
