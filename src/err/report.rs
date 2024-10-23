pub fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

fn report(line: usize, r#where: &str, msg: &str) {
    if line == 0 {
        eprintln!("[System Error]: {msg}");
    } else {
        eprintln!("[line {line}] Error {where}: {msg}");
    }
}
