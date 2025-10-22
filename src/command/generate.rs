use crate::{command::CommandExecutor, output::Table, parser::ParseNode};

pub struct Generate {}

impl CommandExecutor for Generate {
    fn execute(command: &ParseNode, state: &mut Table) -> Result<(), Box<dyn std::error::Error>> {
        println!("Hello from generate");
        // let count = command.arguments.get("count").unwrap().clone();
        // let count: usize = count.parse()?;

        // let name = command.arguments.get("name").cloned().unwrap_or("$1".into());
    
        // state.clear();
        
        // state.add_col(&name, None);
        
        // for _ in 0..count {
        //     let mut e = Vec::new();
        //     e.push(String::from("<Empty>"));
        //     state.try_add_row(e)?;
        // }
        
        // for i in 0..rows {
        //     println!("{}", i);
        //     let mut col = Vec::new();
        //     if i == 0 {
        //         col.push("$1".into());
        //         col.push("$2".into());
        //         col.push("$3".into());
        //     }
        //     state.push(col);
        // }
        
        // println!("{:?}", state);
        
        // let column_count: usize = 3;

        // for rows in state {
        //     println!("{}", "|----|".repeat(column_count));
        //     if rows.len() == 0 {
        //         print!("{}", "|    |".repeat(column_count));
        //     } else {
        //         for cols in rows {
        //             print!("| {} |", cols);
        //         }
        //     }
        //     println!("");
        // }
        // println!("{}", "|----|".repeat(column_count));

        return Ok(());
    }
}