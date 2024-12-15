use crate::ast::EvaluateValue;
/**
 * As you probably noticed from the file name, this should be using visitor pattern.
 * However, I'm not 100% familiar with rust right now and the advanced
 * Box, Trait, Dyn lifetime stuff is currently too complicated for me right now.
 *
 */
use crate::ast::Expression;
// NOTE: known clippy suggestion bug
// https://github.com/rust-lang/rust-clippy/issues/11940
pub fn format_ast(expr: &Box<dyn Expression>) -> String {
    expr.to_string()
}

pub fn interpret(expr: &Box<dyn Expression>) -> Option<String> {
    if let Some(eval) = expr.evaluate() {
        Some(eval.value)
    } else {
        None
    }
}
