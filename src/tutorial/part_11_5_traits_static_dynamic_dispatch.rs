// when generics are involved the compiler needs to know
// what type to use this is called static dispatch
// for static dispatch compiler generates copies of code
// for all the types implementing the trait

trait Print {
    fn print(&self);
}

impl Print for i32 {
    fn print(&self) {
        println!("I got the value of \"{}\"", self);
    }
}

impl Print for String {
    fn print(&self) {
        println!("I got the value of \"{}\"", self);
    }
}

fn display_static<T: Print>(x: T) {
    x.print();
}

// when compiler cannot decide which type to use during
// compile time it uses dynamic dispatch
// dynamic dispatch  uses trait oject denoted by dyn <trait>

fn display_dynamic(x: &dyn Print) {
    x.print();
}

enum Type {
    I32,
    String,
}

fn choose_type_to_display(ty: Type) -> Box<dyn Print> {
    match ty {
        Type::I32 => Box::new(32),
        Type::String => Box::new(String::from("new_string")),
    }
}

pub fn main() {
    display_dynamic(&23);
    display_dynamic(&String::from("hdasd alsjd"));
    display_static(23);
    display_static(String::from("hdasd alsjd"));
    choose_type_to_display(Type::I32).print();
    choose_type_to_display(Type::String).print();
}
