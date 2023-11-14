/*
    Result Enum
    enum Result<T,E> {
        Ok(T),
        Err(E)
    }

*/

fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Error:Division by zero!"))
    } else {
        Ok(dividend / divisor)
    }
}

pub fn main() {
    println!("9.0/3.0 => {:?}", division(9.0f64, 3.0f64));
    println!("9.0/0.0 => {:?}", division(9.0f64, 0.0f64));
}
