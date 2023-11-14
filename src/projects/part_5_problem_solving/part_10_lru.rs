use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T: Debug + Copy> {
    val: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct DoubleLinkedList<T: Debug + Copy> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T: Debug + Copy> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

impl<T: Debug + Copy> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping :{:?}", self.val);
    }
}

impl<T: Debug + Copy> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node<T>>>) -> T {
        let next1 = (*node).borrow_mut().next.as_ref().map(|x| x.clone());
        let next2 = next1.as_ref().map(|node| node.clone());
        let next3 = next1.as_ref().map(|node| node.clone());
        let next4 = next1.as_ref().map(|node| node.clone());
        let prev2 = (*node).borrow_mut().prev.as_ref().map(|x| x.clone());
        let prev3 = (*node).borrow_mut().prev.as_ref().map(|x| x.clone());
        let prev4 = (*node).borrow_mut().prev.as_ref().map(|x| x.clone());

        if prev3.is_none() {
            self.head = next3;
        }
        if next2.is_none() {
            self.tail = prev3;
        }
        next1.map(|x| (*x).borrow_mut().prev = prev2);
        prev4.map(|x| (*x).borrow_mut().next = next4);
        (*node).borrow().val
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

    fn display(&self) {
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

struct LruCache<T: Debug + Eq + PartialEq + Hash + Copy> {
    map: HashMap<T, Rc<RefCell<Node<T>>>>,
    list: DoubleLinkedList<T>,
    capacity: usize,
}

impl<T: Debug + Eq + PartialEq + Hash + Copy> LruCache<T> {
    fn with_capacity(capacity: usize) -> Self {
        LruCache {
            map: HashMap::new(),
            list: DoubleLinkedList::new(),
            capacity,
        }
    }

    fn use_val(&mut self, val: T) {
        if !self.map.contains_key(&val) {
            if self.map.len() < self.capacity {
                self.list.push_back(val);
                let rc = self.list.tail.as_ref().unwrap().clone();
                self.map.entry(val).or_insert(rc);
            } else {
                self.map
                    .remove(&(self.list.head.as_ref().unwrap().borrow().val));
                self.list.remove_front();
                self.list.push_back(val);
                self.map
                    .insert(val, self.list.tail.as_ref().unwrap().clone());
            }
        } else {
            let node = self.map.get(&val).unwrap();
            self.list.remove_node(node.clone());
            self.list.push_back(val);
            self.map
                .insert(val, self.list.tail.as_ref().unwrap().clone());
        }
    }
}

pub fn main() {
    let mut cache = LruCache::<char>::with_capacity(3);
    cache.use_val('a');
    println!("using a");
    cache.list.display();
    cache.use_val('b');
    println!("using b");
    cache.list.display();
    cache.use_val('c');
    println!("using c");
    cache.list.display();
    cache.use_val('d');
    println!("using d");
    cache.list.display();
    cache.use_val('e');
    println!("using e");
    cache.list.display();
    cache.use_val('d');
    println!("using d");
    cache.list.display();

    let mut cache = LruCache::<char>::with_capacity(4);
    cache.use_val('1');
    println!("using 1");
    cache.list.display();
    cache.use_val('2');
    println!("using 2");
    cache.list.display();
    cache.use_val('3');
    println!("using 3");
    cache.list.display();
    cache.use_val('4');
    println!("using 4");
    cache.list.display();
    cache.use_val('5');
    println!("using 5");
    cache.list.display();
    cache.use_val('4');
    println!("using 4");
    cache.list.display();
    cache.use_val('6');
    println!("using 6");
    cache.list.display();
    cache.use_val('8');
    println!("using 8");
    cache.list.display();
    cache.use_val('9');
    println!("using 9");
    cache.list.display();
}
