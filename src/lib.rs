mod node;

use node::Node;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, data: T) {
        match &mut self.head {
            Some(n) => n.push(data),
            None => self.head = Some(Box::new(Node::new(data))),
        }
    }

    pub fn len(&mut self) -> usize {
        let mut count = 0;
        let mut node = &mut self.head;
        while let Some(n) = node {
            count += 1;
            node = &mut n.next
        }
        count
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head.take() {
            Some(n) => {
                self.head = n.next;
                Some(n.data)
            },
            None => None,
        }
    }
}