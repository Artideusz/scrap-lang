use crate::{command::CommandExecutor, parser::ParseNodeType, scanner::TokenType};

pub struct Where;

impl CommandExecutor for Where {
    fn execute(args: &[crate::parser::ParseNode], state: &mut crate::output::Table) -> Result<(), Box<dyn std::error::Error>> {
        
        // Must be comparison
        let argument = args[0]
            .expect_type(ParseNodeType::Argument)?
            .child(0).unwrap()
            .expect_type(ParseNodeType::Comparison)?;

        // Must be value
        let lval = argument
            .child(0).unwrap()
            .expect_type(ParseNodeType::Value)?;

        // Must be operator
        let operator = argument
            .child(1).unwrap()
            .expect_type(ParseNodeType::ComparisonOperator)?
            .token_type();

        // Must be value
        let rval = argument
            .child(2).unwrap()
            .expect_type(ParseNodeType::Value)?;

        let mut counter = 0;

        loop {
            counter += 1;
            
            if counter >= state.row_count() {
                break;
            }

            let env = state.get_row_env(counter)?;

            let lval_evaluated = <Where as CommandExecutor>::evaluate(lval, &env)?;
            let lval_evaluated = lval_evaluated.token_value().unwrap();
            let rval_evaluated = <Where as CommandExecutor>::evaluate(rval, &env)?;
            let rval_evaluated = rval_evaluated.token_value().unwrap();
            
            let is_criteria_met = match operator {
                TokenType::Eeq => lval_evaluated == rval_evaluated,
                TokenType::Neq => lval_evaluated != rval_evaluated,
                TokenType::Gt  => lval_evaluated.parse::<usize>()? > rval_evaluated.parse::<usize>()?,
                TokenType::Lt  => lval_evaluated.parse::<usize>()? < rval_evaluated.parse::<usize>()?,
                TokenType::Gte => lval_evaluated.parse::<usize>()? >= rval_evaluated.parse::<usize>()?,
                TokenType::Lte => lval_evaluated.parse::<usize>()? <= rval_evaluated.parse::<usize>()?,
                _ => false
            };
            
            if !is_criteria_met {
                state.remove_row(counter)?;
                counter -= 1;
            }
        };

        return Ok(());
    }
}