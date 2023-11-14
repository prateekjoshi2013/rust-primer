pub fn main() {
    // &ref &mut ref are called common pointers
    // smart pointers in rust have metdata and
    // special features unlike common pointers

    /* Box Smart Pointers Use Cases */

    /*----------Use Case 1------------------ */

    // box pointer gives us a way to point to
    // a heap memory stored value

    let heap_value: Box<f64> = Box::new(0.625);
    /*
    heap_value :stored in stack
    0.625 : values stored in heap
    */
    let x: f64 = 0.625;
    println!("The x == heap_value => {}", x == *heap_value);

    let stack_var = 4;
    let stack_ref = &stack_var;

    // here heap_var is owner of the value stored in the heap
    // even if it is a pointer to that value this is diff from & and &mut
    let heap_var: Box<i32> = Box::new(stack_var);
    let stack_var = 5;
    println!("stack_var: {} heap_var: {} ", stack_var, *heap_var);

    let point: Box<(i32, i32)> = Box::new((100, 125));
    println!("point.0 = {} point.1 = {}", point.0, point.1);
    let (x, y) = *point;
    println!("Point x:{} y:{}", x, y);
    println!("Point :{:?}", *point);

    let point: Box<(String, String)> = Box::new(("100".to_string(), "125".to_string()));
    println!("point.0 = {} point.1 = {}", point.0, point.1);
    let (x, y) = *point;
    println!("Point x:{} y:{}", x, y);
    // value moved from point because
    // String does not implement Copy trait
    // println!("Point :{:?}", *point);

    /*----------Use Case 2------------------ */
    // used in cases where the size of a value
    // cannot be determined at compile time

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{:?}", list);

    let list = ImprovedList::Cons(
        1,
        Some(Box::new(ImprovedList::Cons(
            2,
            Some(Box::new(ImprovedList::Cons(3, None))),
        ))),
    );
    println!("{:?}", list);
}

/*creates a List type chain until reaches nil*/

/*

Errors shown by compiler without using Box
- recursive type `List` has infinite size
- recursive without indirection
- insert some indirection to break the cycle: `Box<`, `>`

enum List {
    Cons(i32,List),
    Nil
}
*/

#[derive(Debug)]
enum List {
    // this resolves the problem of indeterminate size
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ImprovedList {
    // this resolves the problem of indeterminate size
    // using Option does not need Nil variant
    Cons(i32, Option<Box<ImprovedList>>),
}
