use crate::ast::expressions::Expression;
use crate::ast::visitors::format_expr_ast;
use std::fmt::Display;

pub trait Statement: Display {
    // Just a helper function
    fn evaluate(&self);
}

impl Statement for PrintStat
where
    Self: Display,
{
    fn evaluate(&self) {
        // What print statements does, is just printing the value
        format_expr_ast(&self.expr);
        if let Some(eval) = self.expr.evaluate() {
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
    fn evaluate(&self) {
        self.expr.evaluate();
    }
}

impl Display for ExpressionStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expr.fmt(f) // NOTE: this is just temporary
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
