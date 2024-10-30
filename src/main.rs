use std::{
    collections::VecDeque, env, f32::consts::E, fs, io::{stdin, stdout, BufRead, Write}, process::ExitCode
};

mod scanner;
use scanner::Scanner;
mod err;
use err::ErrReport;
mod expr;

fn run_file(file_name: &String) -> u8 {
    println!("Run file name: {}", file_name);
    let f_read_result = fs::read_to_string(file_name);
    match f_read_result {
        Ok(f_contents) => {
            if let Err(_) = run(&f_contents) {
                1
            } else {
                0
            }
        }, 
        Err(_) => 1 as u8
    }
}

fn run(line: &String) -> Result<(),Box<dyn ErrReport>> {
    let mut scanner = Scanner::new(String::from(line.trim()).clone());
    match scanner.scan_tokens() {
        Ok(token_vec) => {
            dbg!(token_vec);
            Ok(())
        },
        Err(errs) => {
            for e_err in errs {
                e_err.report();
            }
            Ok(())
        },
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
                if let Err(e) = run(&buffer) {
                    e.as_ref().report();
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
