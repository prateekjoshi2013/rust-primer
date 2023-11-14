pub fn main() {
    let mut sum = 0;
    loop {
        println!(
            "
Choices:
    Enter +ve number to add
    Enter -ve number to show sum
Enter your choice:"
        );
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("invalid input");
        let guess: i32 = choice.trim().parse().expect("invalid input");
        if guess > 0 {
            sum += guess;
            continue;
        }
        break;
    }
    println!("The sum is: {}", sum);

    // equivalent loop

    let last_num = loop {
        println!(
            "
Choices:
    Enter +ve number to continue
    Enter -ve number to show last entered num
Enter your choice:"
        );
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("invalid input");
        let guess: i32 = choice.trim().parse().expect("invalid input");
        if guess > 0 {
            continue;
        }
        break guess;
    };
    println!("The last -ve num: {}", last_num);
}
