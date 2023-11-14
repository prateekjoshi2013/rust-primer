// A trait can inhereit methods and sssociated types from parent trait

trait Person {
    fn name(&self) -> &str;
}

// Student inherits from Person trait
// Any type implemnting Student must
// also implement Person
trait Student: Person {
    fn complete_info(&self) -> (&str, u8, &str);
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent inherits from Programmer & student
// the type implemnting CompSciStudent must implement
// bot Programmer & Student
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct UniStudent {
    name_std: String,
    age: u8,
    university: String,
}

impl Person for UniStudent {
    fn name(&self) -> &str {
        self.name_std.as_str()
    }
}

impl Student for UniStudent {
    fn complete_info(&self) -> (&str, u8, &str) {
        (self.name(), self.age, self.university.as_str())
    }
}

fn info<S: Student>(s: &S) {
    println!("{:?}", s.complete_info());
    println!("{:?}", s.name());
}

// marker traits are traits without
// methods and associated types
// these are used to enforce some
// contraints on a type

trait SomeProperties: Clone + PartialEq + Default {}

#[derive(Default, Clone, PartialEq)]
struct Citizen {
    name: String,
    age: u8,
    nationality: String,
}

impl SomeProperties for Citizen {}

// Auto Traits
// Auto traits are automatiaccly implemented by a type
// if all its members also implement the trait

// since all buit in types implement Default
// Customer struct automatically implements
// Default trait
#[derive(Default)]
struct Customer {
    name: String,
    age: u8,
    relationship: Visit,
}

enum Visit {
    Casual,
    New,
    Frequent,
}

impl Default for Visit {
    fn default() -> Self {
        Self::New
    }
}

pub fn main() {
    let student = UniStudent {
        name_std: "xyz".to_string(),
        age: 82,
        university: "xyz university".to_string(),
    };
    info(&student);

    let _c = Customer::default();
}
