pub fn main() {
    basic_fn();
    function_with_inputs("Prateek", 50000);
    let prod = function_with_inputs_outputs(5, 2);
    println!("multiplication is {}", prod);
    let (prod, sum) = function_with_inputs_mutliple_outputs(5, 2);
    println!("multiplication is {} , sum is {}", prod, sum);

    // code blocks
    let full_name = {
        let first_name = "Prateek";
        let second_name = "Joshi";
        format!("{first_name} {second_name}")
    };
    println!("full name from code block is: {}", full_name);

    // user input
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    //convert into float value
    let n: f64 = n.trim().parse().expect("invalid input");
    println!("read input {}", n);
}

fn basic_fn() {
    println!("this is a basic function");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("name: {},salary: {}", name, salary);
}

fn function_with_inputs_outputs(a: i32, b: i32) -> i32 {
    a * b
}

fn function_with_inputs_mutliple_outputs(a: i32, b: i32) -> (i32, i32) {
    (a * b, a + b)
}
