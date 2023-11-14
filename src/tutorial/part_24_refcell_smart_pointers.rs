use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

pub fn main() {
    /* Features of RefCell
     */
    // - the borrowing refcell are checked at runtime

    // - refcell borrowed references dont go out of
    //   scope by last use like & &mut

    // - infact the borrowed references go out of scope
    //   with the end of code block

    let x = 23;
    let a = RefCell::new(x);
    let b = a.borrow(); // borrowed as a immutable ref
    drop(b);
    let c = a.borrow_mut(); //borrowed as mutable ref at the same time

    /*
        immutable ref b and mutable ref c are pointing to a
        this is allowed at compile time but panics at runtime

        println!("a={:?},b={:?},c={:?}", a, b, c);
    */

    // this prints a=RefCell { value: <borrowed> }
    // since it is being borrowed by c as mutable ref
    println!("a={:?}", a);
    drop(c);
    // this prints a=RefCell { value: 23 }
    // since c mutable ref to a is dropped
    println!("a={:?}", a);

    /*-------------------------------------------- */
    /*
       We can mutate value of a refcell value
       even if it is not through a mutable variable
       for ex a is not a mutable variable
    */
    let a = RefCell::new(10);
    let mut c = a.borrow_mut();
    *c = 11;
    println!("{:?}", a);
    drop(c);
    println!("{:?}", a);

    /*------------------------------------------- */

    let mut a = Rc::new(String::from("My String"));
    let b = a.borrow_mut();
    // cannot borrow as immutable ref
    // because b is a mutable ref to a
    // let c=&a;
    // println!("{:?} {:?}", b, c);

    // This can be achieved using RefCell easily
    // when we wrap a RefCell with an Rc we can have multiple 
    // owners of that data with both mutable and immutable access

    let a = Rc::new(RefCell::new(String::from("Hello")));
    let b = Rc::clone(&a);
    (*b).borrow_mut().push_str(" World");
    let c = Rc::clone(&a);
    (*c).borrow_mut().push_str("!");

    print!("{:?},{:?},{:?}", a, b, c);
}
