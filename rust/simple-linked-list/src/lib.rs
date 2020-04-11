use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut p = &self.head;
        let mut count: usize = 0;

        while p.is_some() {
            count += 1;
            let node: &Node<T> = &*(p.as_ref().unwrap());
            p = &node.next;
        }

        count
    }

    pub fn push(&mut self, element: T) {
        let old_head = self.head.take();
        self.head = match old_head {
            None => Some(Box::new(Node::new(element))),
            Some(node) => {
                let mut new_node = Node::new(element);
                new_node.next = Some(node);
                Some(Box::new(new_node))
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|bx| {
            let node: &Box<Node<T>> = &*bx;
            &node.data
        })
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut head = self.head;

        while head.is_some() {
            let next = head.unwrap();
            reversed.push(next.data);
            head = next.next;
        }

        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = SimpleLinkedList::new();

        for i in iter {
            l.push(i);
        }

        l
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();
        let mut p = self.rev().head;

        while p.is_some() {
            let node = p.unwrap();
            v.push(node.data);
            p = node.next;
        }

        v
    }
}
