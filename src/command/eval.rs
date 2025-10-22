use crate::{command::CommandExecutor, output::Table, parser::ParseNode};

pub struct Eval {}

impl CommandExecutor for Eval {
    fn execute(command: &ParseNode, state: &mut Table) -> Result<(), Box<dyn std::error::Error>> {
        // let kv = command.arguments.iter().next().unwrap();
        // state.add_col(kv.0, Some(kv.1.clone()));

        return Ok(());
    }
}