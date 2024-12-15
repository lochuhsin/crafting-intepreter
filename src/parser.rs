use crate::{
    ast::expressions::{BinaryExpr, Expression, GroupExpr, LiteralExpr, UnaryExpr, UnknownExpr},
    ast::statements::{ExpressionStat, PrintStat, Statement},
    ast::tokens::{Token, TokenType},
    errors::report,
};

/*
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
 * Expressions ----------------------------------------------------------------
 *
 * expression       -> comma
 * comma            -> ternary ( (",") ternary ) *
 * ternary          -> equality (? equality : equality )*  # Should be right associative
 * equality         -> comparison ( ( "!=" | "==") comparison )*
 * comparison       -> term ( ( ">" | ">=" | "<=" | "<") term )*
 * term             -> factor ( ( "-" | "+" ) factor )*
 * factor           -> unary (( "/" | "*" ) unary )*
 * unary            -> ( "!" | "-") unary | primary;
 * primary          -> NUMBER | STRING | "true" | "false" | "nil" | ( expression)
 *
 *
 *
 * Statements ----------------------------------------------------------------
 *
 *
 *
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

    pub fn parse(&mut self) -> Vec<Box<dyn Statement>> {
        let mut statements = Vec::<Box<dyn Statement>>::new();
        while !self.is_at_end() {
            statements.push(self.statement())
        }
        statements
    }

    fn statement(&mut self) -> Box<dyn Statement> {
        if self.match_tokens(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Box<dyn Statement> {
        let expr = self.expression();
        self.consume(&TokenType::Semicolon, "Expect ; after value");
        Box::new(PrintStat::new(expr))
    }

    fn expression_statement(&mut self) -> Box<dyn Statement> {
        let expr = self.expression();
        self.consume(&TokenType::Semicolon, "Expect ; after value");
        Box::new(ExpressionStat::new(expr))
    }

    pub fn parse_expr_for_test(&mut self) -> Box<dyn Expression> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.comma()
    }

    fn comma(&mut self) -> Box<dyn Expression> {
        let mut expr = self.ternary();
        while self.match_tokens(&[TokenType::Comma]) {
            let operator = self.previous();
            let right = self.ternary();
            expr = Box::new(BinaryExpr::new(expr, operator.clone(), right))
        }
        expr
    }

    fn ternary(&mut self) -> Box<dyn Expression> {
        /*
         * Note: LoL, I'm using left associative. In fact
         * this should be written as right associative
         * still figuring out how to implement right associative
         *
         * ((a ? b : c )? b : c)
         */
        let mut expr = self.equality();
        while self.match_tokens(&[TokenType::QuestionMark]) {
            let question = self.previous();
            let mid = self.equality();

            if self.match_tokens(&[TokenType::Colon]) {
            } else {
                parse_error(self.peek(), "Invalid ternary expression, missing Colon");
            }
            let colon = self.previous();
            let right = self.equality();
            let sub_binary = Box::new(BinaryExpr::new(mid, colon, right));
            expr = Box::new(BinaryExpr::new(expr, question, sub_binary));
        }
        expr
    }

    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr: Box<dyn Expression> = self.comparison();
        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            // https://stackoverflow.com/questions/76151846/cannot-borrow-self-as-immutable-because-it-is-also-borrowed-as-mutable-d
            // important, so switch the order of right and operator
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(BinaryExpr::new(expr, operator.clone(), right))
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
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(BinaryExpr::new(expr, operator.clone(), right));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();
        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let token = self.factor();
            expr = Box::new(BinaryExpr::new(expr, operator.clone(), token))
        }
        expr
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();
        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let token = self.unary();
            expr = Box::new(BinaryExpr::new(expr, operator.clone(), token))
        }
        expr
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Box::new(UnaryExpr::new(self.previous().clone(), self.unary()))
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
            Box::new(LiteralExpr::new(self.previous().clone()))
        } else if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(&TokenType::RightParen, "Expect ')' after expression");
            Box::new(GroupExpr::new(expr))
        } else {
            Box::new(UnknownExpr::new())
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
            parse_error(self.peek(), msg);
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

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
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

pub fn parse_error(token: &Token, msg: &str) {
    if *token.get_token_type() == TokenType::EOF {
        report(token.get_line(), "at end".to_string(), msg.to_string());
    } else {
        report(
            token.get_line(),
            format!("at {}", token.get_lexeme()),
            msg.to_string(),
        )
    }
}
