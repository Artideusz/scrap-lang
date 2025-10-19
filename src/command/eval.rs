use crate::{command::CommandExecutor, output::Table};

pub struct Eval {}

impl CommandExecutor for Eval {
    fn execute(command: &crate::interpreter::Command, state: &mut Table) -> Result<(), Box<dyn std::error::Error>> {
        let kv = command.arguments.iter().next().unwrap();
        state.add_col(kv.0, Some(kv.1.clone()));

        return Ok(());
    }
}