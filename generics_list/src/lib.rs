#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            size: 0
        }
    }

    pub fn push(&mut self, value: T) {
        self.size += 1;
        let new_node = Node {
            value: value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.size -= 1;
            self.head = node.next.map(|boxed_node| *boxed_node);
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
    
}