use std::rc::Rc;
use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops : Cell::new(0),
            states : RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.track_worker();
        self.states.borrow_mut().push(false);
        (id, Thread::new_thread(id, c, self))
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
            self.drops.set(self.drops.get() + 1)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid : usize,
    cmd : String,
    parent : &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid : p,
            cmd : c,
            parent : t
        }
    }
    
    pub fn skill(self) {
        self.parent.add_drop(self.pid)
    }
}


// **************************************************


fn main() {
    let worker = Workers::new();
    let (id, thread) = worker.new_worker(String::from("command"));
    let (id1, thread1) = worker.new_worker(String::from("command1"));

    thread.skill();

    println!("{:?}", (worker.is_dropped(id), id, &worker.drops));

    thread1.skill();
    println!("{:?}", (worker.is_dropped(id1), id1, &worker.drops));

    let (id2, thread2) = worker.new_worker(String::from("command2"));
    let thread2 = Rc::new(thread2);
    let thread2_clone = thread2.clone();

    drop(thread2_clone);

    println!("{:?}", (worker.is_dropped(id2), id2, &worker.drops, Rc::strong_count(&thread2)));
}