use std::{
    collections::VecDeque,
    env,
    io::{stdin, stdout, BufRead, Write},
    process::ExitCode
};

mod scanner;
use scanner::Scanner;
mod err;
use err::{report::error, Err};

fn run_file(file_name: &String) -> u8 {
    println!("Run file name: {}", file_name);
    0
}

fn run(line: &String) -> Option<Box<dyn Err>> {
    let mut scanner = Scanner::new(line.clone());
    let token_vec = scanner.scan_tokens();
    match token_vec {
        Ok(tknv) => {
            None
        },
        Result::Err(e) => Some(Box::new(e)),
    }
}

fn run_promt() -> u8 {
    let mut buffer = String::new();
    loop {
        print!("> ");
        _ = stdout().flush();
        match stdin().lock().read_line(&mut buffer) {
            Ok(read_size) => {
                if read_size == 0 {
                    return 0;
                }
                // Perform parsing
                if let Some(e) = run(&buffer) {
                    let (line, msg) = e.as_ref().report_str();
                    error(line, &msg);
                }
                buffer.clear(); // After executing, clear the buffer
            }
            Err(err) => return err.raw_os_error().unwrap_or(255) as u8,
        }
    }
}

fn main() -> ExitCode {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();
    // dbg!(&args);
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        ExitCode::from(64)
    } else if args.len() == 1 {
        ExitCode::from(run_file(&args[0]))
    } else {
        ExitCode::from(run_promt())
    }
}
