pub struct Person {
    name: String,
    age: i32,
    salary: i32,
}

impl Person {
    pub fn new(name: String, age: i32, salary: i32) -> Self {
        Person { name, age, salary }
    }

    pub fn salary_range(&self) {
        if self.salary <= 10_000 {
            panic!("the salary must be greater than 10,000");
        } else if self.salary <= 10_000 || self.salary >= 30_000 {
            panic!("the salary is out of our considered range");
        }
    }
}

mod test {
    #[cfg(test)]
    use super::*;
    #[test]
    #[should_panic(expected = "the salary is out of our considered range")]
    fn person_salary_out_of_range() {
        let some_person = Person::new(String::from("Prateek"), 40, 32_000);
        some_person.salary_range();
    }

    #[test]
    #[should_panic(expected = "the salary must be greater than 10,000")]
    fn person_salary_less_than_1000() {
        let some_person = Person::new(String::from("Prateek"), 40, 1_000);
        some_person.salary_range();
    }
}
