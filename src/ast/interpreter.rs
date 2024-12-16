use crate::ast::environment::Environment;
/**
 * This should be using visitor pattern.
 * However, I'm not 100% familiar with rust right now and the advanced
 * Box, Trait, Dyn lifetime stuff is currently too complicated for me right now.
 *
 */
use crate::ast::expressions::Expression;

use super::statements::Statement;
// NOTE: known clippy suggestion bug
// https://github.com/rust-lang/rust-clippy/issues/11940
pub fn format_expr_ast(expr: &Box<dyn Expression>) -> String {
    expr.to_string()
}

pub struct Interpreter {
    env: Environment,
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn interpret_ast(&mut self, stats: &[Box<dyn Statement>]) {
        // Figure out how to handle runtime errors
        for stat in stats.iter() {
            stat.evaluate(&mut self.env);
        }
    }

    pub fn interpret_expr_ast(&mut self, expr: &Box<dyn Expression>) -> Option<String> {
        if let Some(eval) = expr.evaluate(&mut self.env) {
            Some(eval.value)
        } else {
            None
        }
    }
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {
            env: Environment::new(),
        }
    }
}
