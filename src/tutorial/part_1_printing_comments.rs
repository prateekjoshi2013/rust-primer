pub fn main() {
    // this is our first program
    // this is the second line of comment
    /*
    This is multiline comment
    */
    println!("Hello, world!");
    print!("Hello ");
    println!("{}", 1);
    println!(" \\n is printed while \n is not printed");
    println!(
        "\n doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );
    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "code",
        language = "Rust"
    );
}
