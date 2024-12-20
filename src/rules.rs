use crate::tokens::TokenType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Precedence {
    PrecNone = 0,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary,
}

impl Precedence {
    pub fn from_usize(usize: usize) -> Precedence {
        match usize {
            0 => Precedence::PrecNone,
            1 => Precedence::PrecAssignment,
            2 => Precedence::PrecOr,
            3 => Precedence::PrecAnd,
            4 => Precedence::PrecEquality,
            5 => Precedence::PrecComparison,
            6 => Precedence::PrecTerm,
            7 => Precedence::PrecFactor,
            8 => Precedence::PrecUnary,
            9 => Precedence::PrecCall,
            10 => Precedence::PrecPrimary,
            _ => panic!("Invalid Precedence"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseFn {
    String,
    Literal,
    Number,
    Unary,
    Binary,
    Grouping,
    Null,
}

pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn get_rule(token_type: TokenType) -> Option<Self> {
        match token_type {
            TokenType::LeftParen => Some(ParseRule {
                prefix: ParseFn::Grouping,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::RightParen => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::LeftBrace => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::RightBrace => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Comma => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Dot => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Minus => Some(ParseRule {
                prefix: ParseFn::Unary,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecTerm,
            }),
            TokenType::Plus => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecTerm,
            }),
            TokenType::Semicolon => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Slash => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecFactor,
            }),
            TokenType::Star => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecFactor,
            }),
            TokenType::Bang => Some(ParseRule {
                prefix: ParseFn::Unary,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::BangEqual => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecEquality,
            }),
            TokenType::Equal => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::EqualEqual => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecEquality,
            }),
            TokenType::Greater => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            }),
            TokenType::GreaterEqual => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            }),
            TokenType::Less => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            }),
            TokenType::LessEqual => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            }),
            TokenType::Identifier => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::String => Some(ParseRule {
                prefix: ParseFn::String,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Number => Some(ParseRule {
                prefix: ParseFn::Number,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),

            TokenType::And => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Class => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Else => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::False => Some(ParseRule {
                prefix: ParseFn::Literal,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::For => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Fun => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::If => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Nil => Some(ParseRule {
                prefix: ParseFn::Literal,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Or => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Print => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Return => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Super => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::This => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::True => Some(ParseRule {
                prefix: ParseFn::Literal,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::Var => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::While => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::ParseError => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            TokenType::EOF => Some(ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            }),
            _ => None,
        }
    }
}
