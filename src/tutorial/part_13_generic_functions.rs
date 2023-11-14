fn prod<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

fn sum<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}
pub fn main() {
    let (a, b) = (2, 3);
    println!("a: {} b: {} => a * b = {}", a, b, prod(a, b));
    println!("a: {} b: {} => a + b = {}", a, b, sum(a, b));

    let (a, b) = (2.5, 3.4);
    println!("a: {} b: {} => a * b = {}", a, b, prod(a, b));
    println!("a: {} b: {} => a + b = {}", a, b, sum(a, b));
}
