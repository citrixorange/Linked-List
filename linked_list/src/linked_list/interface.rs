use std::fmt::Display;
use std::cmp::PartialEq;

pub trait ILinkedList<T: Display + PartialEq + Copy> {
    fn insert_front(&mut self, data: T);
    fn insert_back(&mut self, data: T);
    fn remove_front(&mut self) -> Option<T>;
    fn remove_back(&mut self) -> Option<T>;
    fn find(&self, elem: T) -> Option<T>;
    fn remove(&mut self, elem: T) -> Option<T>;
    fn print(&self);
}