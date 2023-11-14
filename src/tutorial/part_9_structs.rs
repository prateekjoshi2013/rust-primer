struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn new() -> Self {
        Person {
            name: String::from(""),
            citizenship: String::from(""),
            age: 0,
            gender: 'M',
            salary: 0,
        }
    }

    fn compute_taxes(&self) -> f32 {
        self.salary as f32 * 0.25
    }
}
pub fn main() {
    let person1 = Person {
        name: String::from("Prateek Joshi"),
        citizenship: String::from("India"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    println!(
        "the structure values are name:{} citizenship:{} age:{} gender:{} salary:{}",
        person1.name, person1.citizenship, person1.age, person1.gender, person1.salary
    );
    println!(
        "the calculated taxes for this year :{}",
        person1.compute_taxes()
    );

    // equivalent call alternative to dot notation

    println!(
        "the calculated taxes for this year :{}",
        Person::compute_taxes(&person1)
    );

    let person2 = Person::new();
    println!(
        "the structure values are name:{} citizenship:{} age:{} gender:{} salary:{}",
        person2.name, person2.citizenship, person2.age, person2.gender, person2.salary
    );

    let person3 = Person {
        name: String::from("Ayush Joshi"),
        ..person1 // rest of the values are moved from person1 to person3
        // this makes only primitive values and values which are not copied to person3 ie person1.name  usable 
        // as non primitive values like citizenship is moved 
    };

    println!(
        "the structure values are name:{} citizenship:{} age:{} gender:{} salary:{}",
        person3.name, person3.citizenship, person3.age, person3.gender, person3.salary
    );


}
