use core::ops::Add;

pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
    
    pub fn push(&mut self, data: T) {
        match &mut self.next {
            Some(n) => n.push(data),
            None => self.next = Some(Box::new(Node {
                data,
                next: None
            })),
        }
    }
}

impl<T> Add for Node<T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        let mut node = &mut self.next;
        while let Some(n) = node {
            node = &mut n.next
        }
        *node = Some(Box::new(rhs));
        self
    }
}