use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, cmd: String) -> (usize, Thread) {
        let id = self.track_worker();
        self.states.borrow_mut().push(false);
        (id, Thread::new_thread(id, cmd, self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap_or(&true)
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if *states.get(id).unwrap_or(&true) {
            panic!("{} is already dropped", id);
        } else {
            states[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Thread<'a> {
        Thread { pid, cmd, parent }
    }

    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
}
