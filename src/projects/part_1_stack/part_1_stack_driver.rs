use crate::projects::part_1_stack::part_0_i32_stack::{display, input, new, pop, push};

pub fn main() {
    let mut stack = new(0);
    loop {
        println!(
            "
--------------Display-------------
1) Initialize stack with size
2) Push Value
3) Pop Value
4) Display Stack
5) Exit         
        "
        );

        let choice = input();
        match choice {
            1 => {
                println!("enter size of stack");
                let capacity = input();
                stack = new(capacity as usize);
            }
            2 => {
                println!("enter a value to push into stack");
                let item = input();
                push(&mut stack, item as i32);
            }
            3 => {
                println!("enter a value to push into stack");
                match pop(&mut stack) {
                    Some(x) => println!("The popped item: {}", x),
                    None => println!("stack is empty !!"),
                }
            }
            4 => {
                display(&mut stack);
            }
            _ => {
                break;
            }
        }
    }
}
