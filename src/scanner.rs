use std::{collections::HashMap, error::Error, fmt::Display, hash::Hash, iter::{self, Enumerate, Map, Peekable}, str::Chars};


#[derive(Debug, Clone, PartialEq)]
pub enum CommandKeyword {
    Generate,
    Eval,
    Where,
    Table,
    Rename,
    Top,
    Stats,
    Remove
}

impl TryFrom<&String> for CommandKeyword {
    type Error = Box<dyn Error>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "generate" => Ok(Self::Generate),
            "eval" => Ok(Self::Eval),
            "where" => Ok(Self::Where),
            "table" => Ok(Self::Table),
            "rename" => Ok(Self::Rename),
            "top" => Ok(Self::Top),
            "stats" => Ok(Self::Stats),
            "remove" => Ok(Self::Remove),
            // Add more commands here
            _ => Err("Unknown".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncKeyword {
    Len,
    Sum,
    Count
}

impl TryFrom<&String> for FuncKeyword {
    type Error = Box<dyn Error>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "len" => Ok(Self::Len),
            "sum" => Ok(Self::Sum),
            "count" => Ok(Self::Count),
            // Add more commands here
            _ => Err("Unknown".into())
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Keyword {
//     Command(CommandKeyword),
//     Func(FuncKeyword)
// }

// impl TryFrom<&String> for Keyword {
//     type Error = Box<dyn Error>;

//     fn try_from(value: &String) -> Result<Self, Self::Error> {
//         if CommandKeyword::try_from(value).is_ok() {
//             return Ok(Keyword::Command(CommandKeyword::try_from(value)?));
//         }
//         if FuncKeyword::try_from(value).is_ok() {
//             return Ok(Keyword::Func(FuncKeyword::try_from(value)?));
//         }
//         return Err("Unknown".into());
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Add,
    Sub,
    Eq,
    Eeq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Separator,
    Comma,
    LParen,
    RParen,
    And,
    Or,
    CommandKeyword(CommandKeyword),
    FuncKeyword(FuncKeyword),
    Identifier,
    Number,
    String,
    Comment,
    SpecialIdentifier,
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub line: usize,
    pub offset: usize
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{:?}({})", self.token_type, val),
            None => write!(f, "{:?}", self.token_type)
        }
    }
}

struct Scanner {
    data: Vec<u8>,
    tokens: Vec<Token>,
    
    _current_index: usize,
    _data_size: usize,
    
    // Debug-specific data
    _line: usize,
    _line_index: usize
}

impl Scanner {
    pub fn new() -> Scanner {
        return Scanner {
            data: Vec::new(), 
            tokens: Vec::new(),
            _current_index: 0, 
            _data_size: 0,
            _line: 1,
            _line_index: 1
        }
    }

    fn populate(&mut self, text: &str) {
        self.data = Vec::from(text.as_bytes());
        self._data_size = self.data.len();
        self._current_index = 0;
    }

    fn peek(&self) -> Option<u8> {
        self.data.get(self._current_index).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let res = self.peek();
        self._current_index += 1;
        self._line_index += 1;
        return res;
    }

    fn add_token(&mut self, token_type: TokenType, value: Option<String>) {
        self.tokens.push(Token {
            token_type,
            value,
            offset: self._line_index,
            line: self._line
        });
    }

    fn handle_equals(&mut self) {
        self.advance(); // We know this is '='

        if let Some(c) = self.peek() {
            if c == b'=' {
                self.add_token(TokenType::Eeq, None);
                self.advance();
            } else {
                self.add_token(TokenType::Eq, None);
            }
        }
    }

    fn handle_comma(&mut self) {
        self.add_token(TokenType::Comma, None);
        self.advance();
    }

    fn handle_seperator(&mut self) {
        self.add_token(TokenType::Separator, None);
        self.advance();
    }

    fn handle_newline(&mut self, should_advance: bool) {
        self._line += 1;
        self._line_index = 0;

        if should_advance == true {
            self.advance();
        }
    }

    fn handle_comment(&mut self) {
        self.advance(); // We know this is "*"
        let mut val: String = String::new();
        loop {
            if let Some(c) = self.advance() {
                if c == b'*' && self.advance().unwrap_or(0) == b'/' {
                    val = val.trim().to_owned();
                    self.add_token(TokenType::Comment, Some(val));
                    break;
                } 
                else if c == b'\n' {
                    self.handle_newline(false);
                } 
                else {
                    val.push(c as char);
                }
            } else {
                break;
            }
        }
    }

    fn handle_slash(&mut self) {
        self.advance();
        if let Some(c) = self.peek() {
            match c {
                b'*' => self.handle_comment(),
                _ => {}
            }
        };
    }

    fn is_alpha(&mut self, val: u8) -> bool {
        // uppercase letters
        if val >= 65 && val <= 90 {
            return true
        }

        // lowercase letters
        if val >= 97 && val <= 122 {
            return true
        }

        return false
    }

    fn is_numeric(&mut self, val: u8) -> bool {
        if val >= 48 && val <= 57 {
            return true
        }

        return false
    }

    fn handle_identifier(&mut self) {
        let mut val: String = String::new();
        loop {
            // println!("AA");
            if let Some(c) = self.peek() {
                if self.is_alpha(c) || self.is_numeric(c) {
                    val.push(c as char);
                    self.advance();
                } else {
                    // If the keyword returns an error, just assume it is an identifier
                    if let Ok(command) = CommandKeyword::try_from(&val) {
                        self.add_token(TokenType::CommandKeyword(command), None);
                    } else if let Ok(func) = FuncKeyword::try_from(&val) {
                        if c == b'(' {
                            self.add_token(TokenType::FuncKeyword(func), None);
                        } else {
                            self.add_token(TokenType::Identifier, Some(val));
                        }
                    } else {
                        self.add_token(TokenType::Identifier, Some(val));
                    }
                    break;
                }
            } else {
                self.add_token(TokenType::Identifier, Some(val));
                break;
            }
        }
    }

    fn handle_number(&mut self) {
        let mut val: String = String::new();
        
        loop {
            if let Some(c) = self.peek() {
                if self.is_numeric(c) == true {
                    val.push(c as char);
                    self.advance();
                } else {
                    self.add_token(TokenType::Number, Some(val));
                    break;
                }
            } else {
                self.add_token(TokenType::Number, Some(val));
                break;
            }
        }
    }

    fn handle_string(&mut self) {
        let mut val: String = String::new();

        // We already know this is the "\"" character, so we advance once
        self.advance();

        loop {
            if let Some(c) = self.advance() {
                if c == b'\"' {
                    self.add_token(TokenType::String, Some(val));
                    break;
                } 
                else if c == b'\n' {
                    self.handle_newline(false);
                }
                else {
                    val.push(c as char);
                }
                continue;
            }

            break;
        }
    }

    fn handle_rest(&mut self) {
        if let Some(c) = self.peek() {
            if self.is_numeric(c) == true {
                self.handle_number();
            } else if self.is_alpha(c) == true {
                self.handle_identifier();
            } else {
                self.advance();
            }
        }
    }

    fn handle_special_identifier(&mut self) {
        let mut val: String = String::new();

        val.push(self.advance().unwrap() as char); // We know this is the '$' character

        loop {
            if let Some(c) = self.peek() {
                if self.is_alpha(c) || self.is_numeric(c) {
                    val.push(c as char);
                    self.advance();
                } else {
                    self.add_token(TokenType::SpecialIdentifier, Some(val));
                    break;
                }
            } else {
                self.add_token(TokenType::SpecialIdentifier, Some(val));
                break;
            }
        }
    }

    fn handle_add(&mut self) {
        self.add_token(TokenType::Add, None);
        self.advance();
    }

    fn handle_sub(&mut self) {
        self.add_token(TokenType::Sub, None);
        self.advance();
    }

    fn handle_lt(&mut self) {
        self.advance(); // We know this is '<'

        if let Some(c) = self.peek() {
            if c == b'=' {
                self.add_token(TokenType::Lte, None);
                self.advance();
            } else {
                self.add_token(TokenType::Lt, None);
            }
        }
    }

    fn handle_gt(&mut self) {
        self.advance(); // We know this is '>'

        if let Some(c) = self.peek() {
            if c == b'=' {
                self.add_token(TokenType::Gte, None);
                self.advance();
            } else {
                self.add_token(TokenType::Gt, None);
            }
        }
    }

    fn handle_lparen(&mut self) {
        self.add_token(TokenType::LParen, None);
        self.advance();
    }

    fn handle_rparen(&mut self) {
        self.add_token(TokenType::RParen, None);
        self.advance();
    }

    fn handle_exclamation(&mut self) {
        self.advance();
        if let Some(c) = self.peek() {
            if c == b'=' {
                self.add_token(TokenType::Neq, None);
                self.advance();
            } // else if !self.is_alphanumeric() { TokenType::Not }
        }
    }

    pub fn scan(&mut self, data: &str) -> Vec<Token> {
        self.populate(data);
        // let mut iterator = text.as_bytes().iter().copied().peekable();
        // let mut tokens: Vec<Token> = Vec::new();

        'state: loop {
            if let Some(c) = self.peek() {
                // print!("{}", char::from(c));
                match c {
                    b'+'  => self.handle_add(),
                    b'-'  => self.handle_sub(),
                    b'<'  => self.handle_lt(),
                    b'>'  => self.handle_gt(),
                    b'('  => self.handle_lparen(),
                    b')'  => self.handle_rparen(),
                    b'='  => self.handle_equals(),
                    b','  => self.handle_comma(),
                    b'|'  => self.handle_seperator(),
                    b'/'  => self.handle_slash(),
                    b'\n' => self.handle_newline(true),
                    b'\"' => self.handle_string(),
                    b'$'  => self.handle_special_identifier(),
                    b'!'  => self.handle_exclamation(),
                    _     => self.handle_rest()
                }
                continue;
            }
            break 'state;
        }

        self.add_token(TokenType::EOF, None);

        // println!("{}", data);
        // println!("{:?}", self.tokens);
        return self.tokens.clone();
    }
}

pub(crate) fn scan(text: &str) -> Vec<Token> {
    let mut ctx = Scanner::new();
    return ctx.scan(text);
}