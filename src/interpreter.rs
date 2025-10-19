use std::{ collections::HashMap, error::Error, fmt::Debug, fs::File, io::{BufRead, BufReader, Read} };

use crate::{command::{self, generate, CommandExecutor}, output::Table, scanner::{self, scan, Keyword, Token, TokenType}};

enum Value {
    Number(i64),
    String(String)
}

#[derive(Debug)]
struct Argument {
    identifier: String,
    value: String
}


#[derive(Debug)]
pub enum Statement {
    ECommand(Command),
    EArgsCommand(ArgListCommand),
    EComment(Comment)
}

// trait Statement: Debug {}

type Arguments = HashMap<String, String>;


#[derive(Debug)]
pub struct Command {
    pub keyword: Keyword,
    pub arguments: Arguments
}

// impl Statement for Command {}

#[derive(Debug)]
struct ArgListCommand {
    keyword: String,
    identifiers: Vec<String>
}

// impl Statement for ArgListCommand {}

#[derive(Debug)]
struct Comment {
    keyword: String,
    arguments: Vec<Argument>
}

// impl Statement for Comment {}


struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    statements: Vec<Statement>
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            tokens: Vec::new(),
            current_index: 0,
            statements: Vec::new()
        };
    }

    fn argument(&mut self) -> Result<(String, String), Box<dyn Error>> {
        let token = self.expect(TokenType::Identifier).unwrap();
        let identifier = token.value.expect("Identifier cannot be empty"); // Fix pls

        self.expect(TokenType::Equal).unwrap();
        let token = self.expect_any().unwrap();
        let val = match token.value {
            Some(value) => value,
            _ => panic!("Unexpected empty identifier (how)")
        };

        return Ok((identifier, val));
    }

    fn arguments(&mut self) -> Result<Arguments, Box<dyn Error>> {
        let mut args: Arguments = HashMap::new();
        loop {
            let token = self.peek();
            if self.is_token(TokenType::Identifier) {
                let arg: (String, String) = self.argument().unwrap();
                args.insert(arg.0, arg.1);
            } else if self.is_token(TokenType::EOF) || self.is_token(TokenType::Seperator) || self.is_token(TokenType::Comment) {
                break;
            } else if token.is_none() {
                return Err(format!("Unexpected overflow").into());
            } else {
                let token = token.unwrap();
                return Err(format!("Did not expect token {} on line {} offset {}", token, token.line, token.offset).into());
            }
        }
        return Ok(args);
    }

    fn command(&mut self) -> Result<(), Box<dyn Error>> {
        let token = self.expect_any().unwrap();

        let keyword: Keyword = match token.token_type {
            TokenType::Keyword(k) => Ok(k),
            _ => Err(format!("Did not expect token {} on line {} offset {}", token, token.line, token.offset))
        }.unwrap();

        let args = self.arguments().unwrap();

        self.statements.push(Statement::ECommand(
            Command { keyword: keyword, arguments: args }
        ));

        return Ok(());
    }

    fn statement(&mut self) -> Result<(), Box<dyn Error>> {
        // statement is either a comment or a command
        if self.is_token(TokenType::Seperator) {
            self.consume();
            // This must be a command
            self.command().unwrap();
        } else if self.is_token(TokenType::Comment) {
            // We don't care about comments for now
            self.consume();
        } else if self.is_token(TokenType::EOF) {
            // After EOF, there is no more tokens
            self.consume();
        } else {
            return Err(format!("Unexpected token").into());
        }

        return Ok(());
    }

    fn peek(&self) -> Option<Token> {
        let token = self.tokens.get(self.current_index).cloned();
        return token;
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.peek();
        self.current_index += 1;
        return token;
    }

    fn is_token(&mut self, token_type: TokenType) -> bool {
        return match self.peek() {
            Some(token) => {
                if token.token_type == token_type {
                    return true;
                }
                return false;
            },
            None => false
        }
    }

    fn expect_any(&mut self) -> Result<Token, Box<dyn Error>> {
        return match self.consume() {
            Some(token) => Ok(token),
            None => Err("Unexpected EOF".into())
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Result<Token, Box<dyn Error>> {
        return match self.peek() {
            Some(token) => {
                if token.token_type == token_type {
                    self.consume();
                    return Ok(token);
                }
                return Err(format!("Did not expect token {} on line {} offset {}", token, token.line, token.offset).into());
            },
            None => Err("Unexpected EOF".into())
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<(), Box<dyn Error>> {
        self.tokens = tokens.clone();
        'state: loop {
            // Token can be either Seperator or Empty
            if self.peek().is_none() {
                break 'state;
            }
            self.statement().unwrap();
        }
        return Ok(());
    }
}

pub fn evaluate_command(command: &Command, state: &mut Table) {
    match command.keyword {
        Keyword::Generate => command::generate::Generate::execute(command, state).unwrap(),
        Keyword::Eval => command::eval::Eval::execute(command, state).unwrap(),
        _ => return
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<(), Box<dyn Error>> {
    let mut output: Table = Table::new();
    let mut ctx = Parser::new();
    ctx.parse(tokens).expect("Cannot parse tokens");
    // println!("{:?}", ctx.statements);
    for statement in ctx.statements {
        match statement {
            Statement::ECommand(command) => evaluate_command(&command, &mut output),
            _ => continue
        }
    }
    output.display();
    return Ok(());
}

/*

    TODO
    - Move the parser in it's own file and add REPL here
    - Undo unwrap and add backtrace information in the Err value
    - Add functionality for showing which token caused an error (add stop position and save file data somewhere)
    - Clean code and fix warnings

*/


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

    return Ok(());
}