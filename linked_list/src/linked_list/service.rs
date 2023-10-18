use std::fmt::Display;
use std::cmp::PartialEq;

use crate::linked_list::interface::ILinkedList;
use crate::linked_list::implementation::linked_list::ConcreteLinkedList;

pub struct LinkedList<T>
where
    T: Display + PartialEq + Copy
{
    list: Box<dyn ILinkedList<T>>
}

impl <T> LinkedList<T>
where
    T: Display + PartialEq + Copy + 'static 
{

    pub fn new() -> Self {
        let list = ConcreteLinkedList::<T>::new();
        Self { 
            list: Box::new(list)
        }
    }

}

impl<T> ILinkedList<T> for LinkedList<T>
where
    T: Display + PartialEq + Copy 
{

    fn insert_front(&mut self, data: T) {
        self.list.insert_front(data);
    }

    fn insert_back(&mut self, data: T) {
        self.list.insert_back(data);
    }

    fn remove_front(&mut self) -> Option<T> {
        return self.list.remove_front();
    }

    fn remove_back(&mut self) -> Option<T> {
        return self.list.remove_back();
    }

    fn find(&self, elem: T) -> Option<T> {
        return self.list.find(elem);
    }

    fn remove(&mut self, elem: T) -> Option<T> {
        return self.list.remove(elem);
    }

    fn print(&self) {
        return self.list.print();
    }

}