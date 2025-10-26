use std::{collections::HashMap, error::Error};

use crate::{output::Table, parser::{ParseNode, ParseNodeType}, scanner::{Token, TokenType}};

pub mod generate;
pub mod eval;

// TODO: Clean up this trait
pub trait CommandExecutor {
    fn execute(args: &[ParseNode], state: &mut Table) -> Result<(), Box<dyn Error>>;

    // Input: Value or Expression node, hashmap of variables.
    // Output: An evaluated Value node.
    fn evaluate(val_or_expr: &ParseNode, env: &HashMap<String, String>) -> Result<ParseNode, Box<dyn Error>> {
        
        // When we have a Value of type Identifier or SpecialIdentifier:
        // - We just subsitute the variable into a value node
        if matches!(val_or_expr.variant, ParseNodeType::Value) {
            let token = val_or_expr.token.as_ref().unwrap();
            let mut token_value = token.value.clone().unwrap();
            if  matches!(token.token_type, TokenType::Identifier) ||
                matches!(token.token_type, TokenType::SpecialIdentifier) {
                let val = token.value.as_ref().unwrap();
                token_value = env.get(val).unwrap().clone();
            }
            // Determine the type of substituted value - Naively parse int and fallback to string if fails
            let token_type = if token_value.parse::<usize>().is_ok() { TokenType::Number } else { TokenType::String };
            
            return Ok(ParseNode { variant: ParseNodeType::Value, children: None, token: Some(Token { token_type: token_type, value: Some(token_value), line: 0, offset: 0}) });
        } else if matches!(val_or_expr.variant, ParseNodeType::Expression) {
            let children = val_or_expr.children();
            // Left node is always a Value
            let lvalue = &children[0];

            // It's possible that the value will be an identifier
            let lvalue = &Self::evaluate(lvalue, &env)?;
            
            // Middle node is always an operator
            let operator = children[1]
                .expect_type(ParseNodeType::ArithmeticOperator)?;
            
            // This will be either a Value or Expression
            let rvalue: &ParseNode = &children[2];

            // If rvalue is an Expression, recursively call and change the node
            let rvalue = &Self::evaluate(rvalue, &env)?;

            // Calculate expression
            return if operator.expect_token_type(TokenType::Add).is_ok() {
                // Ok(String::from("LOL"))
                let ltoken = lvalue.token.as_ref().unwrap();
                let rtoken = rvalue.token.as_ref().unwrap();

                let mut res = String::new();

                if  matches!(ltoken.token_type, TokenType::Number) &&
                    matches!(rtoken.token_type, TokenType::Number) {
                    
                    let lnum: usize = lvalue.token_value().unwrap().parse()?;
                    let rnum: usize = rvalue.token_value().unwrap().parse()?;
                    res.push_str(&(lnum + rnum).to_string());
                    Ok(ParseNode { variant: ParseNodeType::Value, children: None, token: Some(Token { token_type: TokenType::Number, value: Some(res), line: 0, offset: 0 }) })
                } else {
                    res.push_str(lvalue.token_value().unwrap());
                    res.push_str(rvalue.token_value().unwrap());
                    Ok(ParseNode { variant: ParseNodeType::Value, children: None, token: Some(Token { token_type: TokenType::String, value: Some(res), line: 0, offset: 0 }) })
                }
            } else {
                Err("Not implemented".into())
            };
        } else {
            return Err("Not an expression nor a value".into());
        }
    }
}