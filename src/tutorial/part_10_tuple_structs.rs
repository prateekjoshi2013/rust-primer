struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 > self.1 {
            self.0
        } else {
            self.1
        }
    }

    fn smaller(&self) -> i32 {
        if self.0 < self.1 {
            self.0
        } else {
            self.1
        }
    }
}
pub fn main() {
    let number = Numbers(10, 20);
    println!(
        "Numbers 1 greater:{},lesser:{}",
        number.greater(),
        number.smaller()
    )
}
