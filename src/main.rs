use std::{
    env, fs,
    io::{self, Write},
    process::exit,
    sync::{Mutex, MutexGuard},
};

use crate::scanner::Scanner;
mod expr;
mod scanner;
mod token;
mod token_type;

static LOX: Mutex<Lox> = Mutex::new(Lox { had_error: false });

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        LOX.lock().unwrap().run_file(&args[1]);
    } else {
        LOX.lock()
            .unwrap()
            .run_prompt()
            .unwrap_or_else(|e| println!("Error occured trying to run run_prompt: {}", e));
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(&self, file: &String) {
        let input = fs::read_to_string(file)
            .unwrap_or_else(|_| panic!("{}", format!("Failed to run script {}", file).to_owned()));
        self.run(&input);
        if self.had_error {
            exit(65);
        }
    }
    fn run_prompt(&mut self) -> Result<(), String> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            match stdin.read_line(&mut buffer) {
                Ok(it) => it,
                Err(err) => return Err(format!("Error occured reading line: {err}")),
            };
            if buffer.is_empty() {
                return Ok(());
            }
            self.run(buffer.as_str());
            self.had_error = false;
        }
    }

    fn run(&self, input: &str) {
        let mut sc = Scanner::from_str(input);
        sc.scan_tokens();
        for token in sc.tokens {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }
}
