use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T: Debug> {
    val: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct DoubleLinkedList<T: Debug> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T: Debug> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

impl<T: std::fmt::Debug> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping :{:?}", self.val);
    }
}

impl<T: Debug> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, val: T) -> &mut Self {
        let new_head = Rc::new(RefCell::new(Node::new(val)));
        match self.head.take() {
            Some(old_head) => {
                (*old_head).borrow_mut().prev = Some(new_head.clone());
                (*new_head).borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
        self
    }

    pub fn push_back(&mut self, val: T) -> &mut Self {
        let new_tail = Rc::new(RefCell::new(Node::new(val)));
        match self.tail.take() {
            Some(old_tail) => {
                (*new_tail).borrow_mut().prev = Some(old_tail.clone());
                (*old_tail).borrow_mut().next = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
        self
    }

    pub fn remove_front(&mut self) -> &mut Self {
        self.head
            .take()
            .map(|old_head| -> Option<Rc<RefCell<Node<T>>>> {
                let res = match (*old_head).borrow_mut().next.take() {
                    Some(new_head) => {
                        self.head = Some(new_head);
                        self.head.as_ref().take().map(|new_head| {
                            (*(*new_head).clone()).borrow_mut().prev = None;
                        });
                        self.head.clone()
                    }
                    None => {
                        self.head.take();
                        self.tail.take();
                        None
                    }
                };
                (*old_head).borrow_mut().next = None;
                (*old_head).borrow_mut().prev = None;
                res
            });
        self
    }

    pub fn remove_back(&mut self) -> &mut Self {
        self.tail.take().map(|old_tail| {
            let res = match (*old_tail).borrow_mut().prev.take() {
                Some(new_tail) => {
                    self.tail = Some(new_tail);
                    self.tail.as_ref().take().map(|new_tail| {
                        (*(*new_tail).clone()).borrow_mut().next = None;
                    });
                    self.tail.clone()
                }
                None => {
                    self.head.take();
                    self.tail.take();
                    None
                }
            };
            (*old_tail).borrow_mut().prev = None;
            (*old_tail).borrow_mut().next = None;
            res
        });
        self
    }

    pub fn display(&self) {
        let mut curr_node = self.head.clone();
        while !curr_node.is_none() {
            print!("->{:?}", curr_node.as_ref().unwrap().borrow().val);
            curr_node = curr_node.unwrap().borrow().next.clone();
        }
        println!();
    }

    pub fn display_rev(&self) {
        let mut curr_node = self.tail.clone();
        while !curr_node.is_none() {
            print!("->{:?}", curr_node.as_ref().unwrap().borrow().val);
            curr_node = curr_node.unwrap().borrow().prev.clone();
        }
        println!();
    }
}
