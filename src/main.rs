mod interpreter;
mod scanner;
mod parser;
mod command;
mod output;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1)
        .cloned()
        .unwrap_or("./test.scrap".into());

    // crate::interpreter::run_file(&path).unwrap();
    interpreter::repl().unwrap();
}
