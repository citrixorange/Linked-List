use std::fmt::Display;
use std::cmp::PartialEq;

use crate::linked_list::interface::ILinkedList;

struct Node<T: Display + PartialEq + Copy> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T: Display + PartialEq + Copy> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None
        }
    }
}

pub struct ConcreteLinkedList<T: Display + PartialEq + Copy> {
    head: Option<Box<Node<T>>>
}

impl<T: Display + PartialEq + Copy> ConcreteLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }
}

impl<T: Display + PartialEq + Copy> ILinkedList<T> for ConcreteLinkedList<T> {

    fn insert_front(&mut self, data: T) {
        if let Some(_h) = &self.head {
            self.head = Some(Box::new(Node {
                data: data,
                next: self.head.take()
            }));
        } else {
            self.head = Some(Box::new(Node::new(data)));
        }
    }

    fn insert_back(&mut self, data: T) {
        let mut current = &mut self.head;

        while let Some(ref mut node) = current {
            if node.next.is_none() {
                node.next = Some(Box::new(Node::new(data)));
                return;
            }
            current = &mut node.next;
        }

        self.insert_front(data);
    }

    fn remove_front(&mut self) -> Option<T> {
        if let Some(h) = &mut self.head {
            let d = h.data;
            self.head = h.next.take();
            return Some(d);
        } else {
            return None;
        }
    }

    fn remove_back(&mut self) -> Option<T> {

        let mut data = None;

        let mut current = &mut self.head;

        while let Some(ref mut node) = current {

            if node.next.is_none() {
                data = Some(node.data);
                break;
            }

            if node.next.as_ref().unwrap().next.is_none() {
                data = Some(node.next.as_ref().unwrap().data);
                node.next = None;
                return data;
            }

            current = &mut node.next;
        }

        self.head = None;

        return data;

    }

    fn find(&self, elem: T) -> Option<T> {
        
        let mut current = &self.head;
        
        while let Some(ref node) = current {
            if node.data == elem {
                return Some(node.data);
            }
            current = &node.next;
        }

        return None;

    }

    fn remove(&mut self, elem: T) -> Option<T> {

        if let Some(node) = &self.head {
            if node.data == elem {
                return self.remove_front();
            }
        } else {
            return None;
        }

        let mut current = &mut self.head;

        while let Some(ref mut node) = current {

            if node.next.is_some() {
                if node.next.as_ref().unwrap().data == elem {
                    let data = Some(node.next.as_ref().unwrap().data);
                    node.next = node.next.as_mut().unwrap().next.take();
                    return data;
                }
            }

            current = &mut node.next;
        }

        return None;
    }

    fn print(&self) {

        let mut current = &self.head;

        while let Some(ref node) = current {
            println!("Elem: {}", node.data);
            current = &node.next;
        }

    }

}