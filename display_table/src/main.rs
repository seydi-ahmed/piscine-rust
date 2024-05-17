use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() && self.body.is_empty() {
            return Ok(());
        }

        let mut col_widths = Vec::new();
        
        // Calculate the maximum width of each column
        for col in 0..self.headers.len() {
            let mut max_width = self.headers[col].len();
            for row in &self.body {
                if row[col].len() > max_width {
                    max_width = row[col].len();
                }
            }
            col_widths.push(max_width);
        }

        // Helper function to format a cell
        let format_cell = |content: &str, width: usize| {
            let padding = width - content.len();
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                content,
                " ".repeat(right_padding)
            )
        };

        // Print headers
        write!(f, "|")?;
        for (header, &width) in self.headers.iter().zip(&col_widths) {
            write!(f, " {} |", format_cell(header, width))?;
        }
        writeln!(f)?;

        // Print separator line
        write!(f, "|")?;
        for &width in &col_widths {
            write!(f, "{}|", "-".repeat(width + 2))?;
        }
        writeln!(f)?;

        // Print rows
        for row in &self.body {
            write!(f, "|")?;
            for (cell, &width) in row.iter().zip(&col_widths) {
                write!(f, " {} |", format_cell(cell, width))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// *************************************************************************************
// *************************************************************************************
// *************************************************************************************
// *************************************************************************************

// Pour tester le code
fn main() {
    let mut table = Table::new();
    println!("{}", table);
    table.headers = vec![
        String::from("Model"),
        String::from("Piece N°"),
        String::from("In Stock"),
        String::from("Description"),
    ];
    table.add_row(&[
        String::from("model 1"),
        String::from("43-EWQE304"),
        String::from("30"),
        String::from("Piece for x"),
    ]);
    table.add_row(&[
        String::from("model 2"),
        String::from("98-QCVX5433"),
        String::from("100000000"),
        String::from("-"),
    ]);
    table.add_row(&[
        String::from("model y"),
        String::from("78-NMNH"),
        String::from("60"),
        String::from("nothing"),
    ]);
    println!("{}", table);
}


/*

|  Model  |  Piece N°   | In Stock  | Description |
|---------+-------------+-----------+-------------|
| model 1 | 43-EWQE304  |    30     | Piece for x |
| model 2 | 98-QCVX5433 | 100000000 |      -      |
| model y |   78-NMNH   |    60     |   nothing   |

*/