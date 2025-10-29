use std::{ error::Error, fmt::Debug };

use crate::scanner::{Token, TokenType};

// #[derive(Debug)]
// pub enum Statement {
//     ECommand(Command),
//     EArgsCommand(ArgListCommand),
//     EComment(Comment)
// }

// trait Statement: Debug {}


// #[derive(Debug)]
// pub struct Command {
//     pub keyword: Keyword,
//     pub arguments: Arguments
// }

// impl Statement for Command {}


// impl Statement for Comment {}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNodeType {
    Value,
    ArithmeticOperator,
    ComparisonOperator,
    Comparison,
    Expression,
    Assignment,
    Argument,
    Command,
    Comment,
    Statement,
    Query
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub variant: ParseNodeType,
    pub children: Option<Vec<ParseNode>>,
    pub token: Option<Token>
}

// Thanks GPT!
impl ParseNode {
    /// Return child nodes safely
    pub fn children(&self) -> &[ParseNode] {
        self.children.as_deref().unwrap_or(&[])
    }

    /// Get first child if exists
    pub fn first_child(&self) -> Option<&ParseNode> {
        self.children.as_ref()?.first()
    }

    /// Get nth child
    pub fn child(&self, index: usize) -> Option<&ParseNode> {
        self.children.as_ref()?.get(index)
    }

    /// Get the token’s string value (if present)
    pub fn token_value(&self) -> Option<&String> {
        self.token.as_ref()?.value.as_ref()
    }

    /// Get the token's type
    pub fn token_type(&self) -> TokenType {
        self.token.as_ref().unwrap().token_type.clone()
    }

    /// Find the first descendant node of a given type
    pub fn find_child_of_type(&self, kind: ParseNodeType) -> Option<&ParseNode> {
        self.children().iter().find(|c| c.variant == kind)
    }

    pub fn expect_type(&self, kind: ParseNodeType) -> Result<&ParseNode, Box<dyn Error>> {
        if self.variant == kind {
            return Ok(self);
        } else {
            return Err(format!("Unexpected type {:?}", self.variant).into());
        }
    }

    pub fn expect_token_type(&self, kind: TokenType) -> Result<&ParseNode, Box<dyn Error>> {
        let token = &self.token;
        if let Some(token) = token {
            if token.token_type == kind {
                return Ok(self);
            } else {
                return Err("Unexpected token".into());
            }
        } else {
            return Err("Token is empty".into());
        }
    }

    // pub fn walk(&self, )
}

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    parse_tree: ParseNode
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            tokens: Vec::new(),
            current_index: 0,
            parse_tree: ParseNode { variant: ParseNodeType::Query, children: Some(Vec::new()), token: None }
        };
    }

    // fn identifier_argument(&mut self) -> Result<ParseNode, Box<dyn Error>> {
    //     let token = self.expect(TokenType::Identifier).unwrap();
        
    //     if self.is_token(TokenType::Eq) {

    //     }
    //     // // self.expect(TokenType::Equal).unwrap();
    //     // let token = self.expect_any().unwrap();
    //     // let val = match token.value {
    //     //     Some(value) => value,
    //     //     _ => panic!("Unexpected empty identifier (how)")
    //     // };

    //     return Ok(ParseNode { variant: ParseNodeType::Argument, children: None, token: None });
    // }

    // fn function(&mut self) -> Result<ParseNode, Box<dyn Error>> {
    //     let func = self.expect(TokenType::FuncKeyword(_))
    // }

    fn value(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let token = self.expect_any()?;
        let result = ParseNode { variant: ParseNodeType::Value, children: None, token: Some(token.clone()) };
        
        return match token.token_type {
            TokenType::Number |
            TokenType::Identifier |
            TokenType::SpecialIdentifier |
            TokenType::String => Ok(result),
            _ => Err(format!("Unexpected token {:} at line {} offset {}", token, token.line, token.offset).into())
        }
    }

    // fn numeric_argument(&mut self) -> Result<ParseNode, Box<dyn Error>> {
    //     return Ok(ParseNode { variant: ParseNodeType::Argument, children: Some(vec![self.value()?]), token: None });
    // }

    fn get_position(&self) -> usize {
        return self.current_index;
    }

    fn set_position(&mut self, pos: usize) -> Result<(), Box<dyn Error>> {
        if pos > self.tokens.len() {
            return Err("Invalid position".into());
        }

        self.current_index = pos;

        return Ok(());
    }

    fn arithmetic_operator(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let token = self.expect_any()?;
        let result = ParseNode { variant: ParseNodeType::ArithmeticOperator, children: None, token: Some(token.clone()) };
        
        return match token.token_type {
            TokenType::Add |
            TokenType::Sub => Ok(result),
            _ => Err(format!("Unexpected token {:} at line {} offset {}", token, token.line, token.offset).into())
        }
    }

    // VALUE ARITHMETIC_OPERATOR VALUE
    // VALUE ARITHMETIC_OPERATOR EXPRESSION
    fn expression(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let value = self.value()?;
        let operator = self.arithmetic_operator()?;
        
        let pos = self.get_position();
        let node = self.expression()
            .or_else(|_| {
                self.set_position(pos)?;
                return self.value();
            })?;

        return Ok(ParseNode { variant: ParseNodeType::Expression, children: Some(vec![value, operator, node]), token: None });
    }

    // IDENTIFIER EQUAL VALUE
    // IDENTIFIER EQUAL EXPRESSION
    // SPECIAL_IDENTIFIER EQUAL VALUE
    // SPECIAL_IDENTIFIER EQUAL EXPRESSION
    fn assignment(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let mut children = Vec::new();
        
        // if !self.is_token(TokenType::Identifier) {
        //     return Err("Invalid token".into());
        // }

        let identifier = self.value()?;
        let identifier = identifier
            .expect_token_type(TokenType::Identifier)
            .or_else(|_| identifier.expect_token_type(TokenType::SpecialIdentifier))?
            .to_owned();

        children.push(identifier); // Identifier required
        self.expect(TokenType::Eq)?;
        
        let pos = self.get_position();
        let node = self.expression()
            .or_else(|_| {
                self.set_position(pos)?;
                return self.value();
            })?;

        children.push(node);

        return Ok(ParseNode { variant: ParseNodeType::Assignment, children: Some(children), token: None });
    }

    fn comparison_operator(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let token = self.expect_any()?.clone();
        // let token_type = token.token_type.clone();
        
        let result = ParseNode { variant: ParseNodeType::ComparisonOperator, children: None, token: Some(token.clone()) };
        
        return match token.token_type {
            TokenType::Eeq |
            TokenType::Neq |
            TokenType::Lt  |
            TokenType::Lte | 
            TokenType::Gt  |
            TokenType::Gte => Ok(result),
            _ => Err(format!("Unexpected token {:} at line {} offset {}", token, token.line, token.offset).into())
        }
    }

    // VALUE COMPARISON_OPERATOR VALUE
    fn comparison(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let mut children = Vec::new();

        children.push(self.value()?);
        children.push(self.comparison_operator()?);
        children.push(self.value()?);

        return Ok(ParseNode { variant: ParseNodeType::Comparison, children: Some(children), token: None });
    }

    fn argument(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let mut pos = self.get_position();
        let mut nodes = Vec::new();

        loop {
            if self.is_token(TokenType::Comma) || self.is_token(TokenType::Comment) || self.is_token(TokenType::Separator) || self.is_token(TokenType::EOF) {
                break;
            }

            let node = self.assignment()
                .or_else(|_| {
                    self.set_position(pos)?;
                    return self.comparison();
                })
                .or_else(|_| {
                    self.set_position(pos)?;
                    return self.value();
                })?;

            nodes.push(node);
            pos = self.get_position();
        };

        return Ok(ParseNode { variant: ParseNodeType::Argument, children: Some(nodes), token: None })
    }

    fn arguments(&mut self) -> Result<Vec<ParseNode>, Box<dyn Error>> {
        let mut args = Vec::new();
        
        // TODO: Solve mixing of argument types
        loop {
            if self.is_token(TokenType::Separator) || self.is_token(TokenType::Comment) || self.is_token(TokenType::EOF) {
                break;
            }

            args.push(self.argument()?);
        }

        return Ok(args);
    }

    fn command(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        // let token = self.expect(TokenType::Keyword(Keyword::Command(())))
        let token = self.expect_any()?;

        if !matches!(token.token_type, TokenType::CommandKeyword(_)) {
            return Err(format!("Unexpected token {:} at line {} offset {}", token, token.line, token.offset).into());
        }

        let args = self.arguments()?;

        return Ok(ParseNode {
            variant: ParseNodeType::Command,
            children: Some(args),
            token: Some(token)
        });
    }

    fn comment(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        let token = self.expect(TokenType::Comment)?;

        return Ok(ParseNode {
            variant: ParseNodeType::Comment,
            children: None,
            token: Some(token)
        });
    }

    fn statement(&mut self) -> Result<ParseNode, Box<dyn Error>> {
        if self.is_token(TokenType::Separator) {
            self.expect(TokenType::Separator)?;
            return Ok(ParseNode { variant: ParseNodeType::Statement, children: Some(vec![self.command()?]), token: None });
        }

        if self.is_token(TokenType::Comment) {
            return Ok(ParseNode { variant: ParseNodeType::Statement, children: Some(vec![self.comment()?]), token: None });
        }

        let token = self.expect_any()?;
        return Err(format!("Unexpected token {:} at line {} offset {}", token, token.line, token.offset).into());
    }

    fn peek(&self) -> Option<Token> {
        let token = self.tokens.get(self.current_index).cloned();
        // println!("PEEK {:?}", token);
        return token;
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.peek();
        // println!("CONSUME {:?}", token);
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
                return Err(format!("Did not expect token {:} on line {} offset {}", token, token.line, token.offset).into());
            },
            None => Err("Unexpected EOF".into())
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<(), Box<dyn Error>> {
        self.tokens = tokens.clone();
        'state: loop {
            if self.peek().is_none() {
                return Err("Unexpected EOF".into());
            }
            // Token can be either Seperator or Empty
            if self.is_token(TokenType::EOF) {
                break 'state;
            }
            
            let statement_node = self.statement()?;

            if let Some(children) = &mut self.parse_tree.children {
                children.push(statement_node);
            }
            
        }
        return Ok(());
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<ParseNode, Box<dyn Error>> {
    // let mut output: Table = Table::new();
    let mut ctx = Parser::new();
    ctx.parse(tokens)?;

    // println!("{:?}", ctx.parse_tree);
    
    return Ok(ctx.parse_tree.clone()); // Yuck
}
