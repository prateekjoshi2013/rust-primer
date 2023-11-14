pub fn main() {
    /*
       Option enum
       - Some(T)
       - None
    */

    let mut disease = None;
    disease = Some(String::from("Diabetes"));
    match disease {
        Some(disease_name) => println!("You have the desease of {}", disease_name),
        None => println!("You do not have any disease"),
    }
    let some_value = Some("Some Value");
    println!(
        "The value of Some(\"Some Value\"):Option<&str> = {}",
        some_value.unwrap()
    );

    let some_value = "Some Value";
    if let val = some_value {
        println!("The value of Some(\"Some Value\"):Option<&str> = {}", val);
    }
}
