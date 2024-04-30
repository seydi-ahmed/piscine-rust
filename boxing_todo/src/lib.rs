mod err;
use err::{ParseErr, ReadErr};

use json::JsonValue;
pub use json::{parse, stringify};
pub use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Ouvre le fichier et gère les erreurs potentielles de l'ouverture
        let mut file = File::open(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            })
        })?;

        // Lit le contenu du fichier dans une chaîne de caractères
        let mut contents = String::new();
        
        file.read_to_string(&mut contents).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            })
        })?;

        // Parse le contenu JSON
        let parsed =
            json::parse(&contents).map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))))?;

        // Convertit le JSON en TodoList
        let todo_list = parse_json_to_todo_list(parsed)?;

        Ok(todo_list)
    }
}

fn parse_json_to_todo_list(parsed: JsonValue) -> Result<TodoList, Box<dyn Error>> {
    let title = parsed["title"]
        .as_str()
        .ok_or_else(|| Box::new(ParseErr::Empty))?
        .to_string();

    // Vérifie si la liste des tâches est vide
    let tasks = parsed["tasks"]
        .members()
        .map(|t| Task {
            id: t["id"].as_u32().unwrap_or_default(),
            description: t["description"].as_str().unwrap_or_default().to_string(),
            level: t["level"].as_u32().unwrap_or_default(),
        })
        .collect::<Vec<Task>>();

    if tasks.is_empty() {
        return Err(Box::new(ParseErr::Empty));
    }

    Ok(TodoList { title, tasks })
}