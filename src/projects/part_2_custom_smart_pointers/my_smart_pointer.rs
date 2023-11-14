#[derive(Debug)]
struct MySmartPointer<T> {
    val: T,
}

impl<T> MySmartPointer<T> {
    fn new(val: T) -> Self {
        MySmartPointer { val: val }
    }
}

// To enable *my_smart_pointer
impl<T> std::ops::Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> std::ops::DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl<T> Drop for MySmartPointer<T> {
    // This method is called implicitly when the value goes out of scope,
    // and cannot be called explicitly (this is compiler error E0040).
    // However, the [mem::drop] function in the prelude can be used to
    // call the argument's Drop implementation.
    fn drop(&mut self) {
        println!("Dropping MySmartPointer");
    }
}

pub fn main() {
    let a = 50;
    let b: Box<i32> = Box::new(a);
    println!("a==*b => {}", a == *b); // *b

    // custom smart pointer

    let a = 50;
    let mut b = MySmartPointer { val: a };
    println!("a==*b => {}", a == *b); // *b -> deref()
    *b = 51; // derefMut() is being called
    println!("{:?} has value *b={}", b, *b);

    // drop() called here
}
