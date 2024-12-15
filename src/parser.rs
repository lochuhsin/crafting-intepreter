use crate::{
    ast::{Binary, Expression, Grouping, Literal, Unary, UnknownExpression},
    errors,
    tokens::{Token, TokenType},
};

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
 * // a, b, c, d, e
// expression (comma expression)*
// (expression comma) * expression

 * expression       -> comma
 * comma ->         -> equality ( (",") equality ) *
 * equality         -> comparison ( ( "!=" | "==") comparison )*
 * comparison       -> term ( ( ">" | ">=" | "<=" | "<") term )*
 * term             -> factor ( ( "-" | "+" ) factor )*
 * factor           -> unary (( "/" | "*" ) unary )*
 * unary            -> ( "!" | "-") unary | primary;
 * primary          -> NUMBER | STRING | "true" | "false" | "nil" | ( expression)
 */

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Box<dyn Expression> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.comma()
    }

    fn comma(&mut self) -> Box<dyn Expression> {
        let mut expr = self.equality();
        while self.match_tokens(&[TokenType::Comma]) {
            let right = self.equality();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right))
        }
        expr
    }

    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr: Box<dyn Expression> = self.comparison();
        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            // https://stackoverflow.com/questions/76151846/cannot-borrow-self-as-immutable-because-it-is-also-borrowed-as-mutable-d
            // important, so switch the order of right and operator
            let right = self.comparison();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right))
        }
        expr
    }

    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr: Box<dyn Expression> = self.term();
        while self.match_tokens(&[
            TokenType::GreaterEqual,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
            let right = self.term();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();
        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let token = self.factor();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), token))
        }
        expr
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();
        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let token = self.unary();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), token))
        }
        expr
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Box::new(Unary::new(self.previous().clone(), self.unary()))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::String,
            TokenType::Number,
        ]) {
            Box::new(Literal::new(self.previous().clone()))
        } else if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(&TokenType::RightParen, "Expect ')' after expression");
            Box::new(Grouping::new(expr))
        } else {
            Box::new(UnknownExpression::new())
        }
    }

    // Tools for implementing the grammar above
    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        let mut flag = false;
        for token_type in token_types.iter() {
            if self.check(token_type) {
                self.advance();
                flag = true;
                break;
            }
        }
        flag
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            errors::parse_error(self.peek(), msg);
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().get_token_type() == token_type
        }
    }

    fn is_at_end(&self) -> bool {
        *self.peek().get_token_type() == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        if !self.is_at_end() {
            self.current += 1
        }
        token
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        // will be used with statements
        self.advance();
        while !self.is_at_end() {
            if self.previous().get_token_type() == &TokenType::Semicolon {
                return;
            }
            match *self.peek().get_token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
        }
    }
}
