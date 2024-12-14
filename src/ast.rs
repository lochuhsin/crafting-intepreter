use crate::tokens::Token;
/* The basic concept of Context Free Grammar
 * expression -> literal | unary | binary | grouping
 * literal -> Number | String | "true" | "false" | "nil"
 * grouping -> "(" expression ")"
 * unary -> ("-" | "!") expression
 * binary -> expression operator expression
 * operator -> "==" | "!=" ... etc
 *
 * The actual grammar expression that we would use:
 *
 * Note: We would need to define the Precedence for different operators
 * and the associativity for evaluation (left associative of right associative).
 *
 * Each rule here only matches the expressions at its precedence level or higher.
 *
 * Equality: ==, !=
 * Comparison: >, >=, <=, <
 * Term: -, +
 * Factor: /, *
 * Unary: !, -
 *
 * expression       -> equality
 * equality         -> comparison ( ( "!=" | "==") comparison )*
 * comparison       -> term ( ( ">" | ">=" | "<=" | "<") term )*
 * term             -> factor ( ( "-" | "+" ) factor )*
 * factor           -> unary (( "/" | "*" ) unary )*
 * unary            -> ( "!" | "-") unary | primary;
 * primary          -> NUMBER | STRING | "true" | "false" | "nil" | ( expression)
 */

pub trait Expression {
    fn is_expr(&self);
    fn format_string(&self) -> String;
}

impl Expression for Literal {
    fn is_expr(&self) {}
    fn format_string(&self) -> String {
        self.token.get_literal()
    }
}

impl Expression for Grouping {
    fn is_expr(&self) {}
    fn format_string(&self) -> String {
        format!("( grouping {} )", self.expr.format_string().as_str())
    }
}

impl Expression for Unary {
    fn is_expr(&self) {}
    fn format_string(&self) -> String {
        format!(
            "( {} {} )",
            self.operator.get_lexeme(),
            self.right.format_string().as_str()
        )
    }
}

impl Expression for Binary {
    fn is_expr(&self) {}
    fn format_string(&self) -> String {
        format!(
            "( {} {} {} )",
            self.operator.get_lexeme(),
            self.left.format_string().as_str(),
            self.right.format_string().as_str()
        )
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
    pub left: Token,
    pub expr: Box<dyn Expression>,
    pub right: Token,
}

impl Grouping {
    pub fn new(left: Token, expr: impl Expression + 'static, right: Token) -> Self {
        let e = Box::new(expr);
        Grouping {
            left,
            expr: e,
            right,
        }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Unary {
    pub fn new(operator: Token, right: impl Expression + 'static) -> Self {
        let e = Box::new(right);
        Unary { operator, right: e }
    }
}

pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Binary {
    pub fn new(
        left: impl Expression + 'static,
        token: Token,
        right: impl Expression + 'static,
    ) -> Self {
        let l = Box::new(left);
        let r = Box::new(right);
        Binary {
            left: l,
            operator: token,
            right: r,
        }
    }
}
