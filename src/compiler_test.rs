#[cfg(test)]
mod test {
    use crate::chunk::Chunk;
    use crate::compiler::Parser;
    use crate::scanner::Scanner;
    use crate::tokens::TokenType;

    #[test]
    fn scan_string() {
        let chunk = Chunk::default();
        let mut scanner = Scanner::new("\"abcde\"".to_string());
        scanner.scan_token();
        let tokens = scanner.get_tokens();

        let mut parser = Parser::new(tokens.clone(), chunk);
    }
}
