#[derive(Debug)]
struct Customer {
    name: String,
    username: String,
    age: u8,
    is_lifetime_member: bool,
}

struct CustomerBuilder {
    name: String,
    username: Option<String>,
    age: Option<u8>,
    is_lifetime_member: Option<bool>,
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            username: None,
            age: None,
            is_lifetime_member: None,
        }
    }
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }
    fn age(&mut self, age: u8) -> &mut CustomerBuilder {
        self.age = Some(age);
        self
    }

    fn is_lifetime_member(&mut self, is_lifetime_member: bool) -> &mut CustomerBuilder {
        self.is_lifetime_member = Some(is_lifetime_member);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
            is_lifetime_member: self.is_lifetime_member.unwrap_or_default(),
        }
    }
}

pub fn main() {
    let customer = Customer::new("Prateek".to_string())
        .age(32)
        .username("prateek@123".to_string())
        .is_lifetime_member(false)
        .build();
    println!("{:?}", customer);
}
