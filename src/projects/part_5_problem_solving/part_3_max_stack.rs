use std::i32::MIN;

struct MaxStack {
    stack: Vec<i32>,
    max: i32,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            stack: Vec::new(),
            max: MIN,
        }
    }
    fn push(&mut self, price: i32) {
        if self.max <= price {
            self.stack.push(self.max);
            self.max = price;
        }
        self.stack.push(price);
    }

    fn pop(&mut self) {
        if let Some(top) = self.stack.pop() {
            if self.max == top {
                self.stack.pop().map(|x| {
                    self.max = x;
                });
            }
        }
    }

    fn peek_max(&mut self) -> i32 {
        self.max
    }
}

pub fn main() {
    let stock_prices = vec![55, 80, 120, 99, 22, 140, 145];
    let mut max_stack = MaxStack::new();
    for stock_price in stock_prices.iter() {
        max_stack.push(*stock_price);
    }
    let mut idx = 1;
    while !max_stack.stack.is_empty() {
        println!("max for week n-{} => {}", idx, max_stack.max);
        max_stack.pop();
        idx += 1;
    }
}
