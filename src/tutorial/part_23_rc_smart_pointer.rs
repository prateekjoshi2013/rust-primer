/*
    - Value pointed by Reference Counting Rc Pointer
      can be owned by multiple pointers
    - this is used for graph data structures and doubly
      linked list where a node is pointed and owned by
      multiple nodes
*/

/*
    a---
         \
          c---d---e---nil
         /
    b---
*/

use std::rc::Rc;

#[derive(Debug)]
enum List {
    // this resolves the problem of indeterminate size
    Cons(char, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
pub fn main() {
    let c = Rc::new(Cons(
        'c',
        Rc::new(Cons('d', Rc::new(Cons('e', Rc::new(Nil))))),
    ));
    {
        let d = Cons('d', Rc::clone(&c));

        println!(
            "d: {:?} with reference count for c list:{}",
            d,
            Rc::strong_count(&c)
        );

        let e = Cons('e', c.clone());
        println!(
            "e: {:?} with reference count for c list:{}",
            e,
            Rc::strong_count(&c)
        );
    } // here list d and e are out of scope and hence cleared
    println!("reference count for c list:{}", Rc::strong_count(&c));
}
