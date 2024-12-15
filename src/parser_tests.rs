#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use crate::{
        ast::expressions::ExpressionType,
        ast::tokens::{Token, TokenType},
    };

    fn gen_num_token(value: usize) -> Token {
        Token::new(TokenType::Number, value.to_string(), "".to_string(), 1)
    }

    fn gen_str_token(value: &str) -> Token {
        Token::new(TokenType::Number, value.to_string(), "".to_string(), 1)
    }

    fn gen_eof() -> Token {
        Token::new(TokenType::EOF, "".to_string(), "".to_string(), 1)
    }

    fn gen_by_operator(op: TokenType) -> Token {
        Token::new(op, op.as_string().to_string(), "".to_string(), 1)
    }

    #[test]
    fn parse_binary_with_num() {
        let tokens = vec![
            gen_num_token(1),
            gen_by_operator(TokenType::EqualEqual),
            gen_num_token(2),
            gen_eof(),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_for_test();
        assert_eq!(expr.expr_type(), ExpressionType::Binary);
    }
    #[test]
    fn parse_binary_with_string() {
        let tokens = vec![
            gen_str_token("1"),
            gen_by_operator(TokenType::BangEqual),
            gen_str_token("2"),
            gen_eof(),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_for_test();
        assert_eq!(expr.expr_type(), ExpressionType::Binary);
    }

    #[test]
    fn parse_grouping() {
        let tokens = vec![
            gen_by_operator(TokenType::LeftParen),
            gen_by_operator(TokenType::RightParen),
            gen_eof(),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_for_test();
        assert_eq!(expr.expr_type(), ExpressionType::Grouping);
    }

    #[test]
    fn parse_literal() {
        let tokens = vec![gen_str_token("abcde"), gen_eof()];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_for_test();
        assert_eq!(expr.expr_type(), ExpressionType::Literal);
    }

    #[test]
    fn parse_unary() {
        let tokens = vec![
            gen_by_operator(TokenType::Bang),
            gen_str_token("abcde"),
            gen_eof(),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_for_test();
        assert_eq!(expr.expr_type(), ExpressionType::Unary);
    }

    #[test]
    fn parse_comma_grammar() {
        let tokens = vec![
            gen_str_token("a"),
            gen_by_operator(TokenType::Comma),
            gen_str_token("a"),
            gen_by_operator(TokenType::Comma),
            gen_str_token("a"),
            gen_by_operator(TokenType::Comma),
            gen_eof(),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_for_test();
        assert_eq!(expr.expr_type(), ExpressionType::Binary);
    }
}
