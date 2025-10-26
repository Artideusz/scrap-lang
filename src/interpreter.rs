

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

use std::{error::Error, fs::File, io::{self, BufReader, Read, Write}, os::linux::raw::stat};

use crate::{command::{self, CommandExecutor}, interpreter, output::Table, parser::{parse, ParseNode, ParseNodeType}, scanner::{scan, CommandKeyword, Token, TokenType}};


struct Interpreter {
    output: Table
}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter { output: Table::new() };
    }

    pub fn execute_value(&mut self, value: &ParseNode) -> Result<(), Box<dyn Error>> {
        return Ok(());
    }

    pub fn execute_expression(&mut self, expression: &ParseNode) -> Result<(), Box<dyn Error>> {
        let children = expression.children();

        let lvalue = if matches!(&children[0].variant, ParseNodeType::Expression) {
            self.execute_expression(&children[0])?
        } else {
            self.execute_value(&children[0])?
        };
        
        let operator = &children[1];

        let rvalue = if matches!(&children[2].variant, ParseNodeType::Expression) {
            self.execute_expression(&children[2])?
        } else {
            self.execute_value(&children[2])?
        };

        return Ok(());
    }

    pub fn execute_assignment(&mut self, argument: &ParseNode) -> Result<(), Box<dyn Error>> {
        let identifier = argument.child(0).unwrap().token_value().unwrap();

        return Ok(());
    }

    // Substitution and reduction happens here
    pub fn execute_argument(&mut self, argument: &ParseNode) -> Result<(), Box<dyn Error>> {
        // Is it an assignment?
        if matches!(argument.variant, ParseNodeType::Assignment) {
            return Ok(self.execute_assignment(argument)?);
        }
        return Ok(());
    }

    pub fn execute_command(&mut self, command_tree: &ParseNode) -> Result<(), Box<dyn Error>> {        
        // Check if parse node is a command
        if !matches!(command_tree.variant, ParseNodeType::Command) {
            return Err("Not a command!".into());
        }

        // All child nodes are arguments
        let args = command_tree.children();

        // for arg in args {
        //     self.execute_argument(arg)?;
        // }


        // let args = command_tree.children.as_ref().unwrap();
        let token = command_tree.token.as_ref().unwrap();

        let command = &token.token_type;

        let _ = match command {
            TokenType::CommandKeyword(CommandKeyword::Generate) => crate::command::generate::Generate::execute(args, &mut self.output).unwrap(),
            TokenType::CommandKeyword(CommandKeyword::Eval) => crate::command::eval::Eval::execute(args, &mut self.output).unwrap(),
            _ => return Ok(())
        };

        return Ok(());
    }

    pub fn execute_statement(&mut self, statement_tree: &ParseNode) -> Result<(), Box<dyn Error>> {
        // Check if parse node is a statement
        if !matches!(statement_tree.variant, ParseNodeType::Statement) {
            return Err("Fuck".into());
        }

        if let Some(child_nodes) = statement_tree.children.as_ref() {
            for child in child_nodes {
                match child.variant {
                    ParseNodeType::Command => self.execute_command(child)?,
                    _ => continue
                }
            }
        }

        return Ok(());
    }

    pub fn execute(&mut self, parse_tree: ParseNode) -> Result<(), Box<dyn Error>> {
        // Check if parse node is a query
        if !matches!(parse_tree.variant, ParseNodeType::Query) {
            return Err("Fuck".into());
        }

        let children = parse_tree.children.as_ref().unwrap();
        
        for child in children {
            self.execute_statement(child)?;
        }

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

    // println!("{:?}", result);

    let mut interpreter = Interpreter::new();
    
    interpreter.execute(result).unwrap();

    interpreter.output.display();

    return Ok(());
}

pub(crate) fn repl() -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Error in stdout flush");
        
        let mut data = String::new();
        
        io::stdin()
            .read_line(&mut data)
            .expect("Error in stdin read");

        let data = data.trim();

        if data == "table" {
            interpreter.output.display();
            continue;
        }

        if data == "clear" {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().expect("Error in stdout flush");
            continue;
        }

        if data == "quit" {
            break;
        }

        if data == "" {
            continue;
        }

        let tokens = scan(data);
        let parse_tree = match parse(&tokens) {
            Ok(k) => k,
            Err(v) => {
                println!("Parser error: {}", v);
                continue;
            }
        };

        match interpreter.execute(parse_tree) {
            Ok(k) => k,
            Err(v) => {
                println!("Interpreter error: {}", v);
                continue;
            }
        };

        interpreter.output.display();
    }

    return Ok(());
}