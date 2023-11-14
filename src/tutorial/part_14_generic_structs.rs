struct Point<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn print_val(&self) {
        println!("Point {{ x: {:?},y: {:?} }}", self.x, self.y);
    }
}

pub fn main() {
    let p = Point { x: 5, y: 5.0f32 };
    p.print_val();
    let p = Point {
        x: "5",
        y: "5.0f32",
    };
    p.print_val();
}
