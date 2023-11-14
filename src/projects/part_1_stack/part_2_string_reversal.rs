use crate::projects::part_1_stack::part_0_char_stack::{display, input, new, pop, push};


pub fn main() {
    println!("Enter string to  be reversed:");
    let input_string = input();
    let mut stack = new(input_string.len());
    for ch in input_string.trim().chars() {
        push(&mut stack, ch);
    }
    let mut rev_string = String::new();
    while let Some(ch) = pop(&mut stack) {
        rev_string.push(ch);
    }
    println!("Reversed String: {}", rev_string);
}
