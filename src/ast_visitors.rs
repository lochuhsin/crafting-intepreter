/**
 * As you probably noticed from the file name, this should be using visitor pattern.
 * However, I'm not 100% familiar with rust right now and the advanced
 * Box, Trait, Dyn lifetime stuff is currently too complicated for me right now.
 *
 */
use crate::ast::Expression;
pub fn format_ast(expr: impl Expression) -> String {
    expr.format_string()
}
