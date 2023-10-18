extern crate linked_list;

use linked_list::{linked_list::service::LinkedList, linked_list::interface::ILinkedList};

fn main() {
    let mut ll_instance:LinkedList<i32> = LinkedList::<i32>::new();
    ll_instance.insert_front(5);
    ll_instance.insert_front(4);
    ll_instance.insert_front(3);
    ll_instance.insert_front(2);
    ll_instance.insert_front(1);
    ll_instance.insert_back(6);
    let elem = ll_instance.find(3);
    println!("Found: {}", elem.unwrap());
    ll_instance.remove(3);
    ll_instance.print();

}
