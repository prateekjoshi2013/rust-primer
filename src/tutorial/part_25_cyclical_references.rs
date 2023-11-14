use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node {
    val: char,
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {}", self.val)
    }
}

struct WeakNode {
    val: char,
    next: Option<Weak<RefCell<WeakNode>>>,
}

impl Drop for WeakNode {
    fn drop(&mut self) {
        println!("Dropping {}", self.val)
    }
}

pub fn main() {
    let a = Rc::new(RefCell::new(Node {
        val: 'a',
        next: None,
    }));
    let b = Rc::new(RefCell::new(Node {
        val: 'b',
        next: Some(Rc::clone(&a)),
    }));
    let c = Rc::new(RefCell::new(Node {
        val: 'c',
        next: Some(Rc::clone(&b)),
    }));
    println!(
        "strong_ref_count: a={} b={} c={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::strong_count(&c)
    );
    (*a).borrow_mut().next = Some(Rc::clone(&c));
    println!(
        "strong_ref_count: a={} b={} c={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::strong_count(&c)
    );
    // creates a cyclical reference

    // a value is only dropped when the strong reference count=1

    // a Rc::clone() creates a strong reference which means the
    // new cloned reference shares ownership with all its reference
    // since a -> c -> b-> a therefore the strong_count never reaches 1
    // not being called because first it tries to drop c which is being
    // held and owned by a hence ignored then b is owned by both a and c
    // hence ignored  and lastly a which is owned by both a and b
    // that is why no node is dropped and hence creates a memory leak

    // this can be resolved by using weak references

    let a = Rc::new(RefCell::new(WeakNode {
        val: 'a',
        next: None,
    }));
    let b = Rc::new(RefCell::new(WeakNode {
        val: 'b',
        next: Some(Rc::downgrade(&a)),
    }));
    let c = Rc::new(RefCell::new(WeakNode {
        val: 'c',
        next: Some(Rc::downgrade(&b)),
    }));
    println!(
        "strong_ref_count: a={} b={} c={} weak_ref_count: a={} b={} c={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::strong_count(&c),
        Rc::weak_count(&a),
        Rc::weak_count(&b),
        Rc::weak_count(&c)
    );
    (*a).borrow_mut().next = Some(Rc::downgrade(&c));
    (*a).borrow_mut()
        .next
        .as_ref()
        .unwrap()
        .upgrade()
        .unwrap()
        .clone()
        .borrow()
        .val;
    println!(
        "strong_ref_count: a={} b={} c={} weak_ref_count: a={} b={} c={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::strong_count(&c),
        Rc::weak_count(&a),
        Rc::weak_count(&b),
        Rc::weak_count(&c)
    );
}
