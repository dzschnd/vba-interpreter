use crate::error::print_err;
use core::iter::Iterator;
use std::env;
use std::io::Error;
use std::time::Instant;

mod error;
mod evaluator;
mod file_reader;
mod lexer;
mod parser;
mod types;

const DEF_FILE: &str = "test.vba";

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 {
        args[1].as_str()
    } else {
        DEF_FILE
    };
    let debug_print = if args.len() >= 3 {
        args[2] == "debug"
    } else {
        false
    };
    match interpret(path, debug_print) {
        Ok(_f) => {
            println!("\nFinished in {:?}", start.elapsed())
        }
        Err(e) => print_err(e),
    }
}

fn interpret(path: &str, debug_print: bool) -> Result<(), Error> {
    let text: String = file_reader::read_file(path)?;
    let tokens = lexer::lex(text.as_str(), debug_print);
    let nodes = parser::parse(&tokens, debug_print)?;
    evaluator::evaluate(nodes, debug_print)?;

    Ok(())
}
