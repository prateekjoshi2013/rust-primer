pub fn input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Invalid Input!!");
    input_str.trim().to_string()
}

pub fn new(capacity: usize) -> Vec<String> {
    let stack: Vec<String> = Vec::with_capacity(capacity);
    stack
}

pub fn push(stack: &mut Vec<String>, item: String) {
    stack.push(item);
}

pub fn peek(stack: &mut Vec<String>) -> Option<&String> {
    stack.last()
}

pub fn pop(stack: &mut Vec<String>) -> Option<String> {
    stack.pop()
}

pub fn display(stack: &mut Vec<String>) {
    for i in stack.iter().rev() {
        print!("{ } ", i);
    }
}
