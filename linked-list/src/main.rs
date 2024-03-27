use std::iter::FromIterator;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut head = &self.head;
        while let Some(node) = head {
            head = &node.next;
            len += 1;
        }
        len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: std::mem::take(&mut self.head),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = std::mem::take(&mut self.head);
        match head {
            Some(mut node) => {
                self.head = std::mem::take(&mut node.next);
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        while let Some(node) = self.pop() {
            list.push(node);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for node in iter {
            list.push(node);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list = self.rev();
        let mut vec = Vec::new();
        while let Some(node) = list.pop() {
            vec.push(node);
        }
        vec
    }
}

fn main() {
    let mut list = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    println!("{:?}", list);
    println!("Poped item from list is: {:?}", list.pop());
    println!("List after popping: ");
    println!("{:?}", list);
    println!("{:?}", list.peek());
    println!("Length of th list is: {:?}", list.len());
    let vec: Vec<_> = list.into();
    println!("List converted to vector: {:?}", vec);
    let list = SimpleLinkedList::from_iter(vec);
    println!("{:?}", list);
    let list = list.rev();
    println!("List after reversing: ");
    println!("{:?}", list);
}
