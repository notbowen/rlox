#![allow(dead_code)]

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;

use crate::errors::ParsingError;
use crate::scanner::Scanner;

mod errors;
mod macros;
mod scanner;
mod token;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: {} [script]", &args[0]);
        process::exit(-1);
    } else if args.len() == 2 {
        let path = &args[1];
        run_file(path);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let code = fs::read_to_string(path).expect("Cannot find file to run");
    match run(code) {
        Ok(_) => {}
        Err(_) => process::exit(65),
    }
}

fn run_prompt() {
    print_flush!("> ");
    let stdin = io::stdin();

    for line_result in stdin.lock().lines() {
        let line = line_result.expect("Unable to read from STDIN");
        let _ = run(line);
        print_flush!("> ");
    }
}

fn run(code: String) -> Result<(), ParsingError> {
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}
