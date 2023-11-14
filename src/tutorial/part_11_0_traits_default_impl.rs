struct Circle {
    radius: f64,
}

struct Rectangle {
    length: f64,
    width: f64,
}

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64 {
        println!("default implementation !!");
        0f64
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
    fn perimeter(&self) -> f64 {
        2f64 * (self.length + self.width)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14f64 * self.radius * self.radius
    }
}

pub fn main() {
    let circle = Circle { radius: 5f64 };
    let rectangle = Rectangle {
        length: 2f64,
        width: 2f64,
    };

    println!(
        "circle area:{} parimeter:{}",
        circle.area(),
        circle.perimeter()
    );
    println!(
        "rectangle area:{} perimeter:{}",
        rectangle.area(),
        rectangle.perimeter()
    )
}
