#[cfg(test)]
mod test {
    use crate::chunk::Chunk;
    use crate::compiler::{compile, expression};
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::vm::disassemble_chunk;

    // #[test]
    // fn compile_string() {
    //     let str = String::from("\"abcde\"");
    //     let mut chunk = Chunk::default();
    //     compile(str, &mut chunk);
    //     disassemble_chunk(&chunk, "scan string");
    // }
    #[test]
    fn numeric_expression() {
        let s = String::from("1 + 2");
        let mut scanner = Scanner::new(s);
        let mut chunk = Chunk::default();
        let mut parser = Parser::new();
        parser.advance(&mut scanner);

        expression(&mut parser, &mut scanner, &mut chunk);
        disassemble_chunk(&chunk, "scan string");
    }
}
