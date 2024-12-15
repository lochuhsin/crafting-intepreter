use std::fmt::Display;

use crate::tokens::Token;

// Just for convenience, since if we pass expression with trait object
// we loose type information. this is a way that somehow we could get
// the type info if needed.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExpressionType {
    Literal,
    Grouping,
    Unary,
    Binary,
    UnknownExpression,
}
impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Literal => "Literal",
            Self::Grouping => "Grouping",
            Self::Unary => "Unary",
            Self::Binary => "Binary",
            Self::UnknownExpression => "Unknown Expression",
        };
        write!(f, "{}", s)
    }
}

pub trait Expression: Display {
    // Just a helper function
    fn expr_type(&self) -> ExpressionType;
}

impl Expression for Literal
where
    Self: Display,
{
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Literal
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.get_lexeme())
    }
}

impl Expression for Grouping
where
    Self: Display,
{
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Grouping
    }
}
impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( grouping {} )", self.expr)
    }
}

impl Expression for Unary
where
    Self: Display,
{
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Unary
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )", self.operator.get_lexeme(), self.right)
    }
}

impl Expression for Binary
where
    Self: Display,
{
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Binary
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( {} {} {} )",
            self.operator.get_lexeme(),
            self.left,
            self.right
        )
    }
}

impl Expression for UnknownExpression
where
    UnknownExpression: Display,
{
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::UnknownExpression
    }
}

impl Display for UnknownExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( found unknown expression )")
    }
}

// Useful for parser to identify errors
#[derive(Default)]
pub struct UnknownExpression {}

impl UnknownExpression {
    pub fn new() -> Self {
        UnknownExpression {}
    }
}

pub struct Literal {
    pub token: Token,
}

impl Literal {
    pub fn new(token: Token) -> Self {
        Literal { token }
    }
}

pub struct Grouping {
    pub expr: Box<dyn Expression>,
}

impl Grouping {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        Grouping { expr }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Unary { operator, right }
    }
}

pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Binary {
    pub fn new(left: Box<dyn Expression>, token: Token, right: Box<dyn Expression>) -> Self {
        Binary {
            left,
            operator: token,
            right,
        }
    }
}
