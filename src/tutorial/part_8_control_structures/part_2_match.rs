pub fn main() {
    let some_number = 100;
    match some_number {
        1 => println!("the number is 1"),
        2 | 3 => println!("the number is 2 or 3"),
        4..=100 => println!("the number is between 4 to 100"),
        _ => println!("the number is greater than 100"),
    }

    let marks = 50;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };

    println!("marks : {} , grade : {}", marks, grade);
}
