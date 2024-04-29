mod err;
use err::{ParseErr, ReadErr};
pub use json::{parse, stringify};
pub use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file_content = std::fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let todo_list: TodoList = serde_json::from_str(&file_content).map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        if todo_list.tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(todo_list)
    }
}
