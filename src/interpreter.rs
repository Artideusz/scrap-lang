

// pub fn evaluate_command(command: &Command, state: &mut Table) {
//     // match command.keyword {
//     //     Keyword::Generate => command::generate::Generate::execute(command, state).unwrap(),
//     //     Keyword::Eval => command::eval::Eval::execute(command, state).unwrap(),
//     //     _ => return
//     // }
// }

// pub fn parse(tokens: &Vec<Token>) -> Result<(), Box<dyn Error>> {
//     let mut output: Table = Table::new();
//     let mut ctx = Parser::new();
//     ctx.parse(tokens).expect("Cannot parse tokens");
//     // println!("{:?}", ctx.statements);
//     for statement in ctx.statements {
//         match statement {
//             Statement::ECommand(command) => evaluate_command(&command, &mut output),
//             _ => continue
//         }
//     }
//     output.display();
//     return Ok(());
// }

/*

    TODO
    - Move the parser in it's own file and add REPL here
    - Undo unwrap and add backtrace information in the Err value
    - Add functionality for showing which token caused an error (add stop position and save file data somewhere)
    - Clean code and fix warnings

*/

use std::{error::Error, fs::File, io::{BufReader, Read}};

use crate::{command::{self, CommandExecutor}, interpreter, output::Table, parser::{parse, ParseNode, ParseNodeType}, scanner::{scan, CommandKeyword, Token}};


struct Interpreter {
    output: Table
}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter { output: Table::new() };
    }

    pub fn execute(&mut self, parse_tree: ParseNode) -> Result<(), Box<dyn Error>> {
        // let children = parse_tree.children.unwrap();
        // for node in children {
        //     if matches!(node.variant, ParseNodeType::Statement) {
        //         let child = node.children.unwrap().get(0).unwrap();
        //         if matches!(child.variant, ParseNodeType::Command) {

        //         }
        //         let command = node.token.clone().unwrap();
        //         let keyword = match command.token_type {
        //             crate::scanner::TokenType::CommandKeyword(k) => Ok(k),
        //             _ => Err("Not a command")
        //         }?.clone();
                
        //         match keyword {
        //             CommandKeyword::Generate => Ok(command::generate::Generate::execute(&node, &mut self.output)?),
        //             _ => Err("Oh shucks")
        //         }?;
        //     }
        // }
        return Ok(());
    }
}


pub(crate) fn run_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Opening file {}", file_path);
    let file = File::open(file_path)
        .expect("Something went wrong when reading the file");

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut data: String = String::new();

    reader.read_to_string(&mut data)
        .expect("Cannot read data to string");

    let tokens: Vec<Token> = scan(&data);
    let result = parse(&tokens).unwrap();

    let mut interpreter = Interpreter::new();
    
    interpreter.execute(result).unwrap();

    return Ok(());
}