pub fn main() {
    /*

    loop {
        println!("This is infinite loop");
    }

    */
    let my_number = 10;
    let mut choice = 11;
    while choice != my_number {
        let mut guess = String::new();
        println!("Guess my number: ");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Error recieving input");
        println!("your guess: {}", guess);
        choice = guess.trim().parse().expect("Could not parse input")
    }
    println!("your guess was correct !")
    

}
