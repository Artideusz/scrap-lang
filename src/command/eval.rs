use std::collections::HashMap;

use crate::{command::CommandExecutor, output::Table, parser::{ParseNode, ParseNodeType}, scanner::TokenType};

pub struct Eval {}

// Modify or create a new column with some value
// Examples:
// | eval x = 20
// | eval y = x - 2
// | eval z = $row + x
// | eval a = z - y + x + $row
impl CommandExecutor for Eval {
    fn execute(args: &[ParseNode], state: &mut Table) -> Result<(), Box<dyn std::error::Error>> {
        // println!("Hello from eval!");

        // Must be assignment
        let argument = args[0]
            .expect_type(ParseNodeType::Argument)?
            .child(0).unwrap()
            .expect_type(ParseNodeType::Assignment)?;
        
        // Must be identifier
        let identifier = argument
            .child(0).unwrap();
        let identifier = identifier
            .expect_type(ParseNodeType::Value)?
            .expect_token_type(TokenType::Identifier)
            .or_else(|_| identifier.expect_token_type(TokenType::SpecialIdentifier))?
            .token_value().unwrap();

        // Either expression or value, this should be calculated on every row
        let val_or_expr = argument
            .child(1).unwrap();

        let val_or_expr = val_or_expr
            .expect_type(ParseNodeType::Expression)
            .or_else(|_| val_or_expr.expect_type(ParseNodeType::Value) )?;
        

        let col_index = state.get_or_create_column(identifier)?;

        let row_count: usize = state.row_count();

        // println!("Damn");

        for i in 1..row_count {
            let env: HashMap<String, String> = state.get_row_env(i)?;
            // println!("{:?}", env);
            let evaluated_val = <Eval as CommandExecutor>::evaluate(val_or_expr, &env)?; // What?
            let evaluated_val = evaluated_val.token_value().unwrap();
            state.set_cell(col_index, i, evaluated_val)?;
        }
        // for (index, cell) in col.into_iter().enumerate() {

        //     let evaluated_val = CommandExecutor::evaluate(val_or_expr, env)
        // }

        // | eval x = y + 4
        // == x = 6

        // state.add_or_mod_col(identifier, fill_col);

        // for arg in args {
        //     let arg_type_node = arg.child(0).unwrap();
            
        //     if !matches!(arg_type_node.variant, ParseNodeType::Assignment) {
        //         return Err("Invalid argument type".into());
        //     }

        //     let identifier = arg_type_node
        //         .expect_child(0)?
        //         .of_type(); //.unwrap().token_value().unwrap();
        //     let value = arg_type_node.child(1).unwrap().token_value();

        //     match identifier.as_str() {
        //         "count" => { count_arg = value.cloned(); },
        //         "name" => { name_arg = value.cloned(); },
        //         _ => continue
        //     }
        // }

        return Ok(());
    }
}