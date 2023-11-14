pub fn main() {
    println!(
        "add function passed to do_twice as a function pointer : do_twice(add,5,6) => {}",
        do_twice(add, 5, 6)
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
/*
Function Pointer : fn

- Function pointers are pointers that point to code, not data.
  They can be called just like functions.

- Like references, function pointers are, among other things,
  assumed to not be null, so if you want to pass a function
  pointer over FFI and be able to accommodate null pointers,
  make your type Option<fn()> with your required signature

- In addition, all safe function pointers implement Fn, FnMut,
 and FnOnce,because these traits are specially known to the
 compiler
*/
fn do_twice(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b) + f(a, b)
}
