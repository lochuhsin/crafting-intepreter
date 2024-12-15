use crate::tokens::Token;

// Just for convenience, since if we pass expression with trait object
// we loose type information. this is a way that somehow we could get
// the type info if needed.
pub enum ExpressionType {
    Literal,
    Grouping,
    Unary,
    Binary,
    UnknownExpression,
}

pub trait Expression {
    fn format_string(&self) -> String;
    // Just a helper function
    fn expr_type(&self) -> ExpressionType;
}

impl Expression for Literal {
    fn format_string(&self) -> String {
        self.token.get_literal()
    }
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Literal
    }
}

impl Expression for Grouping {
    fn format_string(&self) -> String {
        format!("( grouping {} )", self.expr.format_string().as_str())
    }
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Grouping
    }
}

impl Expression for Unary {
    fn format_string(&self) -> String {
        format!(
            "( {} {} )",
            self.operator.get_lexeme(),
            self.right.format_string().as_str()
        )
    }
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Unary
    }
}

impl Expression for Binary {
    fn format_string(&self) -> String {
        format!(
            "( {} {} {} )",
            self.operator.get_lexeme(),
            self.left.format_string().as_str(),
            self.right.format_string().as_str()
        )
    }
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Binary
    }
}

impl Expression for UnknownExpression {
    fn format_string(&self) -> String {
        "( found unknown expression )".to_string()
    }
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::UnknownExpression
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
