pub fn input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Invalid Input!!");
    input_str.trim().to_string()
}

pub fn new(capacity: usize) -> Vec<char> {
    let stack: Vec<char> = Vec::with_capacity(capacity);
    stack
}

pub fn push(stack: &mut Vec<char>, item: char) {
    if stack.len() != stack.capacity() {
        stack.push(item);
    }
}

pub fn pop(stack: &mut Vec<char>) -> Option<char> {
    stack.pop()
}

pub fn display(stack: &mut Vec<char>) {
    print!("top-> ");
    for i in stack.iter().rev() {
        print!("{ } ", i);
    }
}
