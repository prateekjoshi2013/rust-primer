#[derive(Debug)]
struct Student {
    id: u8,
    name: String,
    age: u8,
}

impl Student {
    fn new(name: String) -> Result<Student, String> {
        if name.eq("Default_Name") {
            Ok(Student {
                id: u8::MIN,
                name: "Default_Name".to_string(),
                age: u8::MIN,
            })
        } else {
            Err("Please provide a name".to_string())
        }
    }
}

impl Default for Student {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            age: Default::default(),
        }
    }
}

pub fn main() {
    let student = Student::default();
    println!("Student Default: {:?}", student);
    let student = Student::new("Prateek".to_string()).unwrap_or_default();
    println!("Student Default: {:?}", student);
    let student = Student {
        age: 12,
        ..Default::default()
    };
    println!("Student Default: {:?}", student);
}
