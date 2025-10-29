use std::{collections::HashMap, error::Error, iter::zip};

use crate::parser::ParseNode;

pub struct Table {
    cells: Vec<Vec<String>>, // Rows -> Cols
    max_width: Vec<usize>
}

impl Table {
    pub fn new() -> Self {
        Self {
            cells: vec![vec![]],
            max_width: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.max_width.clear();
    }

    pub fn row_count(&self) -> usize {
        return self.cells.len();
    }

    pub fn col_count(&self) -> usize {
        return self.cells[0].len();
    }

    pub fn set_cell(&mut self, column: usize, row: usize, value: &String) -> Result<(), Box<dyn Error>> {
        self.cells[row][column] = value.clone();
        if value.len() > self.max_width[column] {
            self.max_width[column] = value.len();
        }
        return Ok(());
    }

    // TODO: Add condition index > 0
    pub fn get_row_env(&self, index: usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut result: HashMap<String, String> = HashMap::new();
        let keys = self.cells[0].clone();
        // println!("{:?}", keys);
        let rows = self.cells[index].clone();

        // let mut hax = 0;
        for (key, cell) in zip(keys, rows) {
            result.insert(key, cell);
            // Special identifiers are returned here
            result.insert(String::from("$rowcount"), index.to_string());
        };

        return Ok(result);
    }

    // Creates a column and returns an index to the column
    pub fn create_column(&mut self, val: &String) -> Result<usize, Box<dyn Error>> {
        let _ = &mut self.cells[0].push(val.clone());
        let _ = &mut self.max_width.push(val.len());

        for row in &mut self.cells[1..] {
            row.push(String::new());
        };
        
        return self.get_column(val);
    }

    // Returns the index of the column
    pub fn get_column(&self, name: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let cols = self.cells.first().ok_or("Empty table")?;
        let index = cols.iter().position(|c| c == name).ok_or("No such column")?;
        // Ok(self.cells.iter().map(|row| &row[index]).collect())
        return Ok(index);
    }

    pub fn get_or_create_column(&mut self, val: &String) -> Result<usize, Box<dyn Error>> {
        // Little borrowing hack so rust analyzer doesnt shout at me
        if self.get_column(val).is_ok() {
            return self.get_column(val);
        }

        self.create_column(val)
    }

    pub fn add_col(&mut self, val: &String, fill_col: Option<String>) {
        let row_val = fill_col.unwrap_or("<Empty>".into());
        if self.cells.len() == 0 {
            self.cells.push(Vec::new()); // Headers
        }
        self.cells[0].push(val.clone());
        self.max_width.push(row_val.len());
        
        for cell in &mut self.cells[1..] {
            cell.push(row_val.clone());
        }

        if val.len() > self.max_width[0] {
            self.max_width[0] = val.len();
        }
    }

    pub fn try_add_row(&mut self, val: Vec<String>) -> Result<(), Box<dyn Error>> {
        // Check the width of the columns
        if val.len() != self.cells[0].len() {
            return Err("Naah, wrong size blud".into());
        }

        self.cells.push(val.clone());

        let mut max_width = (0, 0);
        for ev in val.iter().enumerate() {
            if ev.1.len() > max_width.1 {
                max_width = (ev.0, ev.1.len());
            }
        }

        if self.max_width[max_width.0] > max_width.1 {
            self.max_width[max_width.0] = max_width.1;
        }

        return Ok(());
    }

    pub fn remove_row(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        self.cells.remove(index);
        return Ok(());
    }

    pub fn display(&self) {
        let max_width_sum: usize = self.max_width.iter().sum();
        if self.max_width.len() == 0 {
            println!("No table created, use 'generate' to create one");
            return;
        }
        // println!("{} {:?}", max_width_sum, self.max_width);

        println!("+{}+", "-".repeat(max_width_sum - 1 + self.cells[0].len() * 3));

        for eheader in self.cells[0].iter().enumerate() {
            print!("| {}{} ", eheader.1, " ".repeat(self.max_width[eheader.0] - eheader.1.len()));
        }

        println!("|");
        println!("+{}+", "-".repeat(max_width_sum - 1 + self.cells[0].len() * 3));

        for rows in self.cells[1..].iter() {
            for cols in rows.iter().enumerate() {
                print!("| {}{} ", cols.1, " ".repeat(self.max_width[cols.0] - cols.1.len()));
            }
            println!("|");
            println!("+{}+", "-".repeat(max_width_sum - 1 + self.cells[0].len() * 3));
        }

        // return format!("{:?} {:?}", self.cells, self.max_width);
    }
}