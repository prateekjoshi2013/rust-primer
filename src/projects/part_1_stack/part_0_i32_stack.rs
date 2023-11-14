pub fn input() -> isize {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Invalid Input!!");
    let size: isize = input_str.trim().parse().expect("Invalid Input!!");
    size
}

pub fn new(capacity: usize) -> Vec<i32> {
    let stack: Vec<i32> = Vec::with_capacity(capacity);
    stack
}

pub fn push(stack: &mut Vec<i32>, item: i32) {
    if stack.len() != stack.capacity() {
        stack.push(item);
    }
}

pub fn pop(stack: &mut Vec<i32>) -> Option<i32> {
    stack.pop()
}

pub fn display(stack: &mut Vec<i32>) {
    print!("top-> ");
    for i in stack.iter().rev() {
        print!("{ } ", i);
    }
}
