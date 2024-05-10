fn main() {
	let contents = fs::read_to_string("commits.json").unwrap();
	let serialized = json::parse(&contents).unwrap();
	println!("{:?}", commits_per_week(&serialized));
	println!("{:?}", commits_per_author(&serialized));
}


// ********************************************************************

use chrono::*;
use json::*;
use std::collections::HashMap;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut com_per_week : HashMap<String, u32> = HashMap::new();

    for commit in data.members() {

    }

    com_per_week
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut com_per_author : HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        
    }

    com_per_author
}