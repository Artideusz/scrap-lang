use std::error::Error;

use crate::{interpreter::Command, output::Table};

pub mod generate;
pub mod eval;

pub trait CommandExecutor {
    fn execute(command: &Command, state: &mut Table) -> Result<(), Box<dyn Error>>;
}