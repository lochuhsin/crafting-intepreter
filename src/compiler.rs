use crate::chunk::Chunk;
use crate::scanner::Scanner;

pub fn compile(s: String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(s);
    scanner.scan_tokens();
    let tokens = scanner.get_tokens();
    true
}
