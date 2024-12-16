use crate::ast::expressions::Expression;
use crate::ast::interpreter::format_expr_ast;
use crate::ast::tokens::Token;
use std::fmt::Display;

use super::environment::Environment;

pub trait Statement: Display {
    // Just a helper function
    fn evaluate(&self, environment: &mut Environment);
}

impl Statement for PrintStat
where
    Self: Display,
{
    fn evaluate(&self, _e: &mut Environment) {
        // What print statements does, is just printing the value
        format_expr_ast(&self.expr);
        if let Some(eval) = self.expr.evaluate(_e) {
            println!("{}", eval.value)
        }
    }
}

impl Display for PrintStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expr.fmt(f) // NOTE: this is just temporary
    }
}

impl Statement for ExpressionStat
where
    Self: Display,
{
    fn evaluate(&self, _e: &mut Environment) {
        self.expr.evaluate(_e);
    }
}

impl Display for ExpressionStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expr.fmt(f) // NOTE: this is just temporary
    }
}

impl Statement for VariableStat
where
    Self: Display,
{
    fn evaluate(&self, e: &mut Environment) {
        if let Some(expr) = &self.initializer {
            if let Some(v) = expr.evaluate(e) {
                e.define(self.name.get_lexeme(), v)
            }
        }
    }
}

impl Display for VariableStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Not implemented for variable stat yet");
        write!(f, "var {}", self.name.get_lexeme())
    }
}

pub struct PrintStat {
    expr: Box<dyn Expression>,
}

impl PrintStat {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        Self { expr }
    }
}

pub struct ExpressionStat {
    expr: Box<dyn Expression>,
}

impl ExpressionStat {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        Self { expr }
    }
}

pub struct VariableStat {
    name: Token,
    initializer: Option<Box<dyn Expression>>,
}

impl VariableStat {
    pub fn new(name: Token, initializer: Option<Box<dyn Expression>>) -> Self {
        VariableStat { name, initializer }
    }
}
