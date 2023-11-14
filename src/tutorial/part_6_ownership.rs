#![allow(unused_variables)]

pub fn main() {
    /*
       Rust Ownership
       ---------------
       - Each value in Rust has a variable that is called its owner.
       - There can be only one owner at a time.
       - When the owner goes out of scope, the value will be dropped.

    */

    let x = 32.6;
    let y = x; // value is not moved but copied to y
    println!("x: {} y: {}", x, y);
    let s1 = String::from("abc");
    let s2 = s1;
    // println!("s1: {}",s1); error : value of s1 moved not copied to s2

    /*
       Primitives and Non-Primitive types

       - primitives types cannot be empty and have fixed size like
       ints,floats,bool,arrays ,chars ...
       - non-primitives types can be empty and do not have fixed size
       like vectors strings structs ...
       - primitive variables are copied while non primitive variables are moved
    */
    /*
       References
       - &s1 creates a read only reference pointing to value of String s1
       and the ownership in this case remains with s1

    */
    let s1 = String::from("abc");
    let s2 = &s1;
    println!("s1: {} , s2: {}", s1, s2);

    /*
       we can use clone if we want to create a copy of a non primitive variable
    */
    let v1 = vec![1, 2, 3, 4];
    let v2 = v1.clone();
    println!("vector 1: {:?} ,cloned vector: {:?}", v1, v2);

    /*
       Scope

       - the variable defines with in a curly brace is said to be its scope
       - A variable is dropped automatically as  soon as it goes out of scope
    */

    {
        let my_name = String::from("Prateek Joshi");
    }
    // my_name is not in scope and has been dropped after the closing bracket in the block
    // println!("my name : {}",my_name);

    /*
       Memory type:

       Heap ->  non-primitive types
       Stack -> primitive types

    */
    // stack allocation
    let x: i32 = 5;
    /*
    s1 is stored in stack but the value of the variable
    "Some String" is stored in heap
    */
    let s1 = String::from("Some String");
    /*
    s1 is no longer valid and the s2 variable
    in stack becomes owner of "Some String" in heap
     */
    let s2 = s1;
    /*
    s3 points to s2 in stack ,
    where s2 still owns and points to "Some String" in heap
     */
    let s3 = &s2;
    /*
    new String "Some String" is created in heap
    and is owned by s4 in stack
     */
    let s4 = s2.clone();

    /*
    primitive variable var is copied
    into a new variable as the argument of function call
    */
    let var = 23;
    stack_function(var);
    println!(
        "the var:i32={} is still available after the function call",
        var
    );

    let mut var = String::from("Hello ");
    let var_mut_ref = &mut var;
    heap_function_v2(var_mut_ref);

    println!(
        "var is passed as a mutable reference 
    to heap_function_v2 but owner is with variable var {}",
        &var
    );

    heap_function_v1(var);
    /*
       var not available as the String non primitive is moved to
       heap_function_v1 and the argument variable str owns the
       value of var string
    */
    // println!("{}",var)
}

fn stack_function(mut var: i32) {
    var = 0;
    println!(
        "the argument var:i32={} can be marked as mutable 
    because since it is primitive value it is being copied as a new variable ",
        var
    );
}

fn heap_function_v1(mut str: String) {
    str.push_str("world!");
    println!(
        "the argument mut str:String={} can be marked as mutable 
    because it is non-primitive value it is being moved as a new variable 
    to str ",
        str
    );
}

fn heap_function_v2(str: &mut String) {
    str.push_str("world!");
    println!(
        "the argument &mut str:String={} is a mutable reference to var created 
        and owned by var variable declared in main function
        because it is a mutable reference value it is not owned by str",
        str
    );
}
