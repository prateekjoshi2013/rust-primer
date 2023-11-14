mod doubly_linked_list;
mod singly_linked_list;
use doubly_linked_list::DoubleLinkedList;
use singly_linked_list::{LinkedList, Node};
fn doubly_linked_list_driver() {
    let mut dll: DoubleLinkedList<char> = DoubleLinkedList::new();
    dll.push_front('c')
        .push_front('b')
        .push_front('a')
        .push_back('d')
        .push_back('e')
        .push_back('f');
    dll.display();
    dll.remove_back().remove_back();
    dll.remove_front().remove_front();
    dll.display_rev();
    dll.display();
    dll.remove_front().remove_front().remove_front();
    dll.display();
    dll.display_rev();
    dll.remove_front().remove_front().remove_front();
    dll.display();
    dll.display_rev();
    dll.push_front('a')
        .push_front('b')
        .remove_front()
        .remove_back();
    dll.display();
    dll.display_rev();
}

fn singly_linked_list_driver() {
    let mut list = LinkedList::new();

    list.add(1).add(2).add(3).add(4).add(5);
    list.display();
    while let Some(_) = list.peek() {
        list.remove();
    }
    list.display();
    list.add(1).add(2).add(3);
    list.display();

    let mut list = LinkedList::new();
    list.add('a').add('b').add('c');
    list.display();
}

pub fn main() {
    // singly_linked_list_driver();
    doubly_linked_list_driver();
}
