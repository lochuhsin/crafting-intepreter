/**
 * As you probably noticed from the file name, this should be using visitor pattern.
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

pub fn interpret_expr_ast(expr: &Box<dyn Expression>) -> Option<String> {
    if let Some(eval) = expr.evaluate() {
        Some(eval.value)
    } else {
        None
    }
}

pub fn interpret_ast(stats: &[Box<dyn Statement>]) {
    // Figure out how to handle runtime errors
    for stat in stats.iter() {
        stat.evaluate()
    }
}
