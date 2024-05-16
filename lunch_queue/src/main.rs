#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person : Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            node : None,
        }
    }
    pub fn add(&mut self, name: String, discount: i32) {
        self.node = Some(Box::new(Person{name  : name.clone() , discount , next_person : self.node.take()}));
    }
    pub fn invert_queue(&mut self) {
        if self.node.is_some() {
            let mut new_q =Queue::new();
    
            let mut addr = self.node.as_ref().unwrap();
            
            new_q.add(addr.name.clone() , addr.discount);
            while addr.next_person.is_some() {
                addr = addr.next_person.as_ref().unwrap();
                new_q.add(addr.name.clone() , addr.discount);
            }
            self.node = new_q.node;

        }
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut name = String::from("");
        let mut disc = 0;
        if self.node.is_some() {

            self.invert_queue();
            match &self.node {
                Some(v) => {
                    name = v.name.clone();
                    disc = v.discount;
                },
                _ => {}
            }
            self.node  = self.node.as_mut().unwrap().next_person.take();
            self.invert_queue();
            return Some((name , disc))
        }
        None
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        if self.node.is_some() {
            if self.node.as_ref().unwrap().name == name {
                return Some((self.node.as_ref().unwrap().name.clone() , self.node.as_ref().unwrap().discount))
            }
            let mut addr = &self.node.as_ref().unwrap().next_person;
            
            while let Some(val) = addr {
                if val.name == name {
                    return Some((val.name.clone() , val.discount))
                }
                addr = &addr.as_ref().unwrap().next_person
            }

        }
        None
    }
}


// **************************************************

fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);

    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));

    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
    list.invert_queue();
    println!("invert {:?}", list);
}