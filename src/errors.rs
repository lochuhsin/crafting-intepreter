pub fn error(line: usize, msg: String) {
    report(line, String::new(), msg)
}

pub fn report(line: usize, wh: String, msg: String) {
    println!("line[ {} ] Error {} : {}", line, wh, msg);
}
