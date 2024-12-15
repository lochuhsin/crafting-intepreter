use crate::tokens::{Token, TokenType};
use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EvaluationType {
    Number,
    String,
    Bool,
    Nil,
}

impl Display for EvaluationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Number => write!(f, "Number"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

pub struct EvaluateValue {
    pub eval_type: EvaluationType,
    pub is_empty: bool,
    pub value: String,
}
impl EvaluateValue {
    fn new(eval_type: EvaluationType, is_empty: bool, value: String) -> EvaluateValue {
        EvaluateValue {
            eval_type,
            is_empty,
            value,
        }
    }
}

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
    fn evaluate(&self) -> Option<EvaluateValue>;
}

impl Expression for Literal
where
    Self: Display,
{
    fn expr_type(&self) -> ExpressionType {
        ExpressionType::Literal
    }
    fn evaluate(&self) -> Option<EvaluateValue> {
        match *self.token.get_token_type() {
            TokenType::Number => Some(EvaluateValue::new(
                EvaluationType::Number,
                self.token.get_lexeme().is_empty(),
                self.token.get_lexeme(),
            )),
            TokenType::String => Some(EvaluateValue::new(
                EvaluationType::String,
                self.token.get_lexeme().is_empty(),
                self.token.get_lexeme(),
            )),
            TokenType::True | TokenType::False => Some(EvaluateValue::new(
                EvaluationType::Bool,
                self.token.get_lexeme().is_empty(),
                self.token.get_lexeme(),
            )),
            TokenType::Nil => Some(EvaluateValue::new(
                EvaluationType::Nil,
                true,
                self.token.get_lexeme(),
            )),
            _ => {
                runtime_error(
                    format!(
                        "invalid literal: {}",
                        self.token.get_token_type().as_string()
                    )
                    .as_str(),
                );
                None
            }
        }
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
    fn evaluate(&self) -> Option<EvaluateValue> {
        self.expr.evaluate()
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

    fn evaluate(&self) -> Option<EvaluateValue> {
        /*
         * Note, define true, false and handle when evaluate failure
         * and this is slow as fuck...
         */

        if let Some(mut right_eval) = self.right.evaluate() {
            match *self.operator.get_token_type() {
                TokenType::Bang => {
                    let val = !is_truthy(&right_eval);
                    Some(EvaluateValue::new(
                        EvaluationType::Bool,
                        false,
                        val.to_string(),
                    ))
                }
                TokenType::Minus => {
                    if right_eval.eval_type == EvaluationType::Number {
                        let val = -right_eval.value.parse::<f64>().unwrap();
                        right_eval.value = val.to_string();
                        Some(right_eval)
                    } else {
                        None
                    }
                }
                _ => {
                    runtime_error(
                        format!(
                            "invalid operation for unary: {}",
                            self.operator.get_token_type().as_string()
                        )
                        .as_str(),
                    );
                    None
                }
            }
        } else {
            None
        }
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
    fn evaluate(&self) -> Option<EvaluateValue> {
        let l_eval = self.left.evaluate();
        let r_eval = self.right.evaluate();
        if l_eval.is_none() || r_eval.is_none() {
            return None;
        }

        let l = l_eval.unwrap();
        let r = r_eval.unwrap();

        if l.eval_type != r.eval_type {
            runtime_error(
                format!(
                    "TypeError: unsupported operand type(s) for {}: {} and {}",
                    self.operator.get_token_type().as_string(),
                    l.eval_type,
                    r.eval_type,
                )
                .as_str(),
            );
            return None;
        }

        let operator_type = self.operator.get_token_type();
        match l.eval_type {
            EvaluationType::String => Binary::eval_string(&l, &r, operator_type),
            EvaluationType::Number => Binary::eval_number(&l, &r, operator_type),
            EvaluationType::Nil => Binary::eval_nil(&l, &r, operator_type),
            EvaluationType::Bool => Binary::eval_bool(&l, &r, operator_type),
        }
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

    fn evaluate(&self) -> Option<EvaluateValue> {
        None
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
    pub value: String,
}

impl Literal {
    pub fn new(token: Token) -> Self {
        Literal {
            value: token.get_lexeme(),
            token,
        }
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

    fn eval_string(
        l: &EvaluateValue,
        r: &EvaluateValue,
        operator: &TokenType,
    ) -> Option<EvaluateValue> {
        match *operator {
            TokenType::Plus => {
                let mut new_s = l.value.to_owned();
                let new_s2 = r.value.to_owned();
                new_s.push_str(&new_s2);
                Some(EvaluateValue::new(
                    EvaluationType::String,
                    new_s.is_empty(),
                    new_s,
                ))
            }
            TokenType::BangEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l.value == r.value).to_string(),
            )),
            TokenType::EqualEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l.value != r.value).to_string(),
            )),
            /*
            TODO: Support string comparison .... ?
            like <, >, <=, >= (lexicographic comparison)
            */
            _ => {
                runtime_error(
                    format!(
                        "TypeError: unsupported operand type(s) for {}: {} and {}",
                        operator.as_string(),
                        l.eval_type,
                        r.eval_type,
                    )
                    .as_str(),
                );
                None
            }
        }
    }

    fn eval_bool(
        l: &EvaluateValue,
        r: &EvaluateValue,
        operator: &TokenType,
    ) -> Option<EvaluateValue> {
        // True = 1, False = 0
        let l_bool = l.value.parse::<bool>().unwrap();
        let r_bool = r.value.parse::<bool>().unwrap();
        match *operator {
            TokenType::Plus => {
                let mut val = 0;
                if l_bool {
                    val += 1;
                }
                if r_bool {
                    val += 1;
                }
                Some(EvaluateValue::new(
                    EvaluationType::Number,
                    false,
                    val.to_string(),
                ))
            }
            TokenType::Minus => {
                let mut val = 0;
                if l_bool {
                    val += 1;
                }
                if r_bool {
                    val -= 1;
                }
                Some(EvaluateValue::new(
                    EvaluationType::Number,
                    false,
                    val.to_string(),
                ))
            }
            TokenType::Star => {
                let mut val = 0;
                if l_bool && r_bool {
                    val = 1;
                }
                Some(EvaluateValue::new(
                    EvaluationType::Number,
                    false,
                    val.to_string(),
                ))
            }
            TokenType::Slash => {
                if !r_bool {
                    runtime_error("ValueError: Division by zero is not allowed");
                    // Division by zero runtime Error
                    None
                } else {
                    let mut val = 0;
                    if l_bool {
                        val = 1
                    }
                    Some(EvaluateValue::new(
                        EvaluationType::Number,
                        false,
                        val.to_string(),
                    ))
                }
            }
            TokenType::EqualEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_bool == r_bool).to_string(),
            )),
            TokenType::BangEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_bool != r_bool).to_string(),
            )),
            TokenType::Less => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                ((l_bool as u8) < (r_bool as u8)).to_string(),
            )),
            TokenType::LessEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                ((l_bool as u8) <= (r_bool as u8)).to_string(),
            )),
            TokenType::Greater => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                ((l_bool as u8) > (r_bool as u8)).to_string(),
            )),
            TokenType::GreaterEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                ((l_bool as u8) >= (r_bool as u8)).to_string(),
            )),
            _ => {
                runtime_error(
                    format!(
                        "TypeError: unsupported operand type(s) for {}: {} and {}",
                        operator.as_string(),
                        l.eval_type,
                        r.eval_type,
                    )
                    .as_str(),
                );
                None
            } // runtime error
        }
    }

    fn eval_number(
        l: &EvaluateValue,
        r: &EvaluateValue,
        operator: &TokenType,
    ) -> Option<EvaluateValue> {
        let l_f = l.value.parse::<f64>().unwrap();
        let r_f = r.value.parse::<f64>().unwrap();
        match *operator {
            TokenType::Plus => Some(EvaluateValue::new(
                EvaluationType::Number,
                false,
                (l_f + r_f).to_string(),
            )),
            TokenType::Minus => Some(EvaluateValue::new(
                EvaluationType::Number,
                false,
                (l_f - r_f).to_string(),
            )),
            TokenType::Star => Some(EvaluateValue::new(
                EvaluationType::Number,
                false,
                (l_f * r_f).to_string(),
            )),
            TokenType::Slash => {
                if r_f == 0_f64 {
                    // Division by zero runtime Error
                    runtime_error("ValueError: Division by zero is not allowed");
                    None
                } else {
                    Some(EvaluateValue::new(
                        EvaluationType::Number,
                        false,
                        (l_f / r_f).to_string(),
                    ))
                }
            }
            TokenType::EqualEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_f == r_f).to_string(),
            )),
            TokenType::BangEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_f != r_f).to_string(),
            )),
            TokenType::Less => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_f < r_f).to_string(),
            )),
            TokenType::LessEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_f <= r_f).to_string(),
            )),
            TokenType::Greater => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_f > r_f).to_string(),
            )),
            TokenType::GreaterEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                (l_f >= r_f).to_string(),
            )),
            _ => {
                runtime_error(
                    format!(
                        "TypeError: unsupported operand type(s) for {}: {} and {}",
                        operator.as_string(),
                        l.eval_type,
                        r.eval_type,
                    )
                    .as_str(),
                );
                None
            } // runtime error unsupported operation
        }
    }

    fn eval_nil(
        l: &EvaluateValue,
        r: &EvaluateValue,
        operator: &TokenType,
    ) -> Option<EvaluateValue> {
        match *operator {
            TokenType::EqualEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                false,
                true.to_string(),
            )),
            TokenType::BangEqual => Some(EvaluateValue::new(
                EvaluationType::Bool,
                true,
                false.to_string(),
            )),
            _ => {
                runtime_error(
                    format!(
                        "TypeError: unsupported operand type(s) for {}: {} and {}",
                        operator.as_string(),
                        l.eval_type,
                        r.eval_type,
                    )
                    .as_str(),
                );
                None
            } // runtime error unsupported operation
        }
    }
}

fn is_truthy(eval: &EvaluateValue) -> bool {
    if eval.is_empty {
        false
    } else {
        match eval.eval_type {
            EvaluationType::Bool => eval.value.parse::<bool>().unwrap(),
            EvaluationType::Number => eval.value.parse::<f64>().unwrap() != 0_f64,
            EvaluationType::String => eval.value.parse::<String>().unwrap() != "",
            EvaluationType::Nil => false,
        }
    }
}

fn runtime_error(msg: &str) {
    println!("Runtime error: {}", msg)
}
