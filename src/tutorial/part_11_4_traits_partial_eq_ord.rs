#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
    earning: u32,
    saving: u32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.age == other.age
            && self.earning == other.earning
            && self.saving == other.saving
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.age.partial_cmp(&other.age) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.earning.partial_cmp(&other.earning) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.saving.partial_cmp(&other.saving)
    }
}

// for sorting , collections and match 
// we need to implement marker traits Ord and Eq as well


pub fn main() {
    let bob = Person {
        name: "Bob".to_string(),
        age: 30,
        earning: 30_0000,
        saving: 50_000,
    };

    let mut bob2 = bob.clone();
    bob2.saving = 50_001;
    println!("{:?}", bob2 == bob); // requires partial eq
    println!("{:?}", bob2 <= bob); // requires partial ord
    println!("{:?}", bob2 >= bob); // requires partial ord


}
