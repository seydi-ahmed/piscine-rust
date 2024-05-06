use std::collections::HashMap;
use std::cell::RefCell;

mod messenger;
use crate::messenger::messenger::Logger;
use crate::messenger::messenger::Tracker;

pub struct Worker<'a> {
    track_value: RefCell<Tracker<'a, Worker<'a>>>,
    mapped_messages: HashMap<&'static str, String>,
    all_messages: Vec<String>,
}

impl<'a> Worker<'a> {
    pub fn new(track_value: Tracker<'a, Worker<'a>>) -> Self {
        Worker {
            track_value: RefCell::new(track_value),
            mapped_messages: HashMap::new(),
            all_messages: Vec::new(),
        }
    }
}

impl<'a> Logger for Worker<'a> {
    fn warning(&self, msg: &str) {
        self.mapped_messages.insert("warning", msg.to_string());
        self.all_messages.push(msg.to_string());
    }

    fn info(&self, msg: &str) {
        self.mapped_messages.insert("info", msg.to_string());
        self.all_messages.push(msg.to_string());
    }

    fn error(&self, msg: &str) {
        self.mapped_messages.insert("error", msg.to_string());
        self.all_messages.push(msg.to_string());
    }
}
