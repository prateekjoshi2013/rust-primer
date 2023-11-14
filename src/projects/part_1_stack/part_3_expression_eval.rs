use crate::projects::part_1_stack::part_0_string_stack::{
    display, input, new, peek, pop, push,
};
fn scan_input(input_str: String) -> Vec<String> {
    let mut scanned_token = String::new();
    let mut scanned_tokens: Vec<String> = Vec::new();
    for ch in input_str.chars() {
        match ch {
            '0'..='9' => {
                scanned_token.push(ch);
            }
            _ => {
                if !scanned_token.is_empty() {
                    scanned_tokens.push(scanned_token.to_string());
                }
                scanned_token.clear();
                scanned_tokens.push(ch.to_string());
            }
        }
    }
    if !scanned_token.is_empty() {
        scanned_tokens.push(scanned_token.to_string());
    }

    scanned_tokens
}

fn operator_priority(op: &str) -> u8 {
    match op {
        "^" => 3,
        "/" | "*" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn postfix_expression_scan(scanned_tokens: Vec<String>) -> Vec<String> {
    let mut postfix_expr: Vec<String> = Vec::new();
    let mut postfix_stack = new(0);
    for token in scanned_tokens {
        match token.as_str() {
            "-" | "+" | "/" | "*" | "^" => {
                while let Some(top_token) = peek(&mut postfix_stack) {
                    if operator_priority(token.as_str()) <= operator_priority(top_token.as_str()) {
                        postfix_expr.push(pop(&mut postfix_stack).unwrap());
                    } else {
                        break;
                    }
                }
                push(&mut postfix_stack, token);
            }
            "(" => {
                push(&mut postfix_stack, token);
            }
            ")" => {
                while let Some(top_token) = peek(&mut postfix_stack) {
                    if top_token.as_str() != "(" {
                        postfix_expr.push(pop(&mut postfix_stack).unwrap());
                    } else {
                        pop(&mut postfix_stack).unwrap();
                        break;
                    }
                }
            }
            _ => postfix_expr.push(token),
        }
    }

    while !postfix_stack.is_empty() {
        postfix_expr.push(pop(&mut postfix_stack).unwrap());
    }
    postfix_expr
}

fn postfix_expr_evaluator(postfix_expr: Vec<String>) -> String {
    let mut eval_stack = new(postfix_expr.len());
    for token in postfix_expr {
        match token.as_str() {
            "+" | "-" | "/" | "*" | "^" => {
                let op2: i32 = pop(&mut eval_stack)
                    .unwrap()
                    .parse()
                    .expect("Invalid Operand!");
                let op1: i32 = pop(&mut eval_stack)
                    .unwrap()
                    .parse()
                    .expect("Invalid Operand!");
                match token.as_str() {
                    "+" => push(&mut eval_stack, format!("{}", op1 + op2)),
                    "-" => push(&mut eval_stack, format!("{}", op1 - op2)),
                    "*" => push(&mut eval_stack, format!("{}", op1 * op2)),
                    "/" => push(&mut eval_stack, format!("{}", op1 / op2)),
                    "^" => push(&mut eval_stack, format!("{}", op1.pow(op2 as u32))),
                    _ => panic!("Invalid Operator !"),
                }
            }
            _ => {
                push(&mut eval_stack, token);
            }
        }
    }
    pop(&mut eval_stack).unwrap()
}

pub fn main() {
    let input_str = input();
    println!("input expression: {}", input_str);
    let scanned_tokens = scan_input(input_str);
    println!("scanned tokens {:?}", scanned_tokens);
    let postfix_expr = postfix_expression_scan(scanned_tokens);
    println!("postfix expression: {:?}", postfix_expr);

    println!(
        "Evaluated expression result: {}",
        postfix_expr_evaluator(postfix_expr)
    );
}
