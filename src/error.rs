use core::panic;

pub fn error(line: usize, message: String) {
    report(line, String::new(), message);
}
fn report(line: usize, loc: String, message: String) {
    eprintln!("[line {line}] Error {loc}: {message}");
    panic!();
}
