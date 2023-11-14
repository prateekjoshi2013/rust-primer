// Rust compiler automatically converts
// certain types in input function out of box
// to make the function accept more type

// &String -> &str
fn print_str(str: &str) {
    println!("{:?}", str);
}
// &vec<T> -> &[T]
fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

#[derive(Debug)]
struct A {
    value: i32,
}
// &Box<T> -> &T
fn print_ref<T: std::fmt::Debug>(x: &T) {
    println!("{:?}", x)
}
pub fn main() {
    let my_string = "My String".to_string();
    print_str(&my_string); // &String -> &str
    print_str(&"my static string"); // &str

    let my_vec = vec![1, 2, 3];
    print_vec(&my_vec); // &vec<T> -> &[T]
    let my_arr = ['a'; 5];
    print_vec(&my_arr); // &[char]

    let boxed_val = Box::new(A { value: 5 });
    print_ref(&boxed_val); // &Box<T> -> &T
    let unboxed_val = A { value: 5 };
    print_ref(&unboxed_val); //&A
}
