use std::error::Error;

pub struct Table {
    cells: Vec<Vec<String>>,
    max_width: Vec<usize>
}

impl Table {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            max_width: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
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

    pub fn display(&self) {
        let max_width_sum: usize = self.max_width.iter().sum();
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