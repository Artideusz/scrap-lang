mod interpreter;
mod scanner;
mod parser;
mod command;
mod output;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1)
        .cloned();

    if let Some(path) = path {
        crate::interpreter::run_file(&path).unwrap();
    } else {
        interpreter::repl().unwrap();
    }    
}
