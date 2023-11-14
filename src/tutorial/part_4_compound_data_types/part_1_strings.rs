#![allow(unused_variables)]

pub fn main() {
    /*
        Strings
        - &str: string literals, string slice pointer to fixed size strings
        - String: String Object , variable length strings
    */

    let some_string: &str = "Fixed length string";
    let mut growable_string: String = String::from("This string will grow");
    growable_string.push('x');
    growable_string.pop();
    growable_string.push_str(" growing!!");

    println!(
        "
Basics function on Strings:
    is_empty(): {},
    length: {},
    Bytes: {},
    Contains 'use' : {}, 
    ",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use"),
    );

    growable_string.push_str("     ");

    // trimming
    let str_len = growable_string.trim().len();
    println!(
        "original string: {} trimmed string with length :{}",
        growable_string, str_len
    );

    // number to String
    let num = 6;
    let num_str: String = num.to_string();
    println!("from num:{} to num_str:{}", num, num_str);

    // char to String
    let ch = 'a';
    println!("from char:{} to string:{}", ch, ch.to_string());

    // &str to String
    println!("from &str to String: {}", "hello world !".to_string());

    // empty string
    let empty_string = String::new();
    println!("length of empty string: {}", empty_string.len());

    // formatted strings
    let formatted_string = format!(
        "My first name is {} and my last name is {}",
        "Prateek", "Joshi"
    );

    println!("formatted string is: {}", formatted_string);
}
