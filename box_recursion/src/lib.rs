#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role: role,
            name: name,
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            None => None,
            Some(worker) => {
                self.grade = worker.next;
                Some(worker.name)
            }
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            None => None,
            Some(worker) => Some((worker.name.clone(), worker.role.clone())),
        }
    }
}
