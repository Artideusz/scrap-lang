use crate::{command::CommandExecutor, output::Table, parser::{ParseNode, ParseNodeType}};

pub struct Generate {}

impl CommandExecutor for Generate {
    fn execute(args: &[ParseNode], state: &mut Table) -> Result<(), Box<dyn std::error::Error>> {
        // println!("Hello from generate");
        // parse the args
        let mut count_arg: Option<String> = None;
        let mut name_arg: Option<String> = None;

        // println!("{:?}", args);
        for arg in args[0].children() {
            let arg_type_node = arg;
            
            if !matches!(arg_type_node.variant, ParseNodeType::Assignment) {
                return Err("Invalid argument type".into());
            }

            let identifier = arg_type_node.child(0).unwrap().token_value().unwrap();
            let value = arg_type_node.child(1).unwrap().token_value();

            match identifier.as_str() {
                "count" => { count_arg = value.cloned(); },
                "name" => { name_arg = value.cloned(); },
                _ => continue
            }
        }

        // println!("count={:?} - name={:?}", count_arg, name_arg);

        // let count = command.arguments.get("count").unwrap().clone();
        let count: usize = count_arg.unwrap().parse()?;
        // let col_pos: usize = state.col_count() + 1;

        state.clear();

        // println!("{:?}", name_arg);

        let name = name_arg.unwrap_or(format!("${}", 1).into());
        
        state.add_col(&name, None);
        
        for _ in 0..count {
            let mut e = Vec::new();
            e.push(String::from("<Empty>"));
            state.try_add_row(e)?;
        }
        
        return Ok(());
    }
}