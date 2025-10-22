use std::error::Error;

use crate::{output::Table, parser::ParseNode};

pub mod generate;
pub mod eval;

pub trait CommandExecutor {
    fn execute(command: &ParseNode, state: &mut Table) -> Result<(), Box<dyn Error>>;
}