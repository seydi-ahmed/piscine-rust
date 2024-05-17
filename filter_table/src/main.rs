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

	pub fn filter_col<F>(&self, filter: T) -> Option<Self> where F: Fn(&str) -> bool, {
        let mut new_headers = Vec::new();
        let mut col_index = Vec::new();

        for (index, header) self.headers.iter().enumerate() {
            new_headers.push(header);
            col_index.push(index);
        }

        if new_headers.is_empty() {
            return None;
        }

        let new_body : Vec<Vec<String>> = body.iter()
            .map(|row| col_index.iter().map(|&i| row[i].clone().collect()))
            .collect();

        Some(
            Table {
                headers: new_headers,
                body: new,
            }
        )
	}

	pub fn filter_row<F>(&self, col_name: &str, filter: T) -> Option<Self> where F: Fn(&str) -> bool, {
        
	}
}


// ********************************************************************************
// ********************************************************************************
// ********************************************************************************
// ********************************************************************************


fn main() {
    let mut table = Table::new();
    table.headers = vec![
        "Name".to_string(),
        "Last Name".to_string(),
        "ID Number".to_string(),
    ];
    table.add_row(&[
        "Adam".to_string(),
        "Philips".to_string(),
        "123456789".to_string(),
    ]);
    table.add_row(&[
        "Adamaris".to_string(),
        "Shelby".to_string(),
        "1111123456789".to_string(),
    ]);
    table.add_row(&[
        "Ackerley".to_string(),
        "Philips".to_string(),
        "123456789".to_string(),
    ]);
    let filter_names = |col: &str| col == "Name";
    println!("{:?}", table.filter_col(filter_names));

    let filter_philips = |lastname: &str| lastname == "Philips";
    println!("{:?}", table.filter_row("Last Name", filter_philips));
}