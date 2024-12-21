#[cfg(test)]
mod test {
    use crate::chunk::Chunk;
    use crate::compiler::compile;
    use crate::vm::disassemble_chunk;

    #[test]
    fn compile_string() {
        let str = String::from("\"abcde\"");
        let mut chunk = Chunk::default();
        compile(str, &mut chunk);
        disassemble_chunk(&chunk, "scan string");
    }
}
