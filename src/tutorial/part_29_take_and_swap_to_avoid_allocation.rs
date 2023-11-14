use std::mem;

pub fn main() {
    let mut m = String::from("my string");

    // take replaces original value pointed by m
    // with default value of String type ""
    // and puts "my string" in a new localtion
    // pointed by n
    let mut n = mem::take(&mut m);
    println!("m={:?} n={:?}", m, n);
    // swaps values pointed by &mut m and &mut n
    mem::swap(&mut m, &mut n);
    println!("m={:?} n={:?}", m, n);
    let old_string = mem::replace(&mut m, String::from("my replaced string"));
    println!(
        "replaced string {:?} pointed by m by the new string value {:?}",
        old_string, m
    );
}
