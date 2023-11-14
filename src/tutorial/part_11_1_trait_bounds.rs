// Trait bounds restrict the types that can be used

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

// by binding generic parameter to Double we
// tell compiler that only type implementing
// Double trait can use quaruple function
fn quadruple<T: Double>(x: T) -> T {
    x.double().double()
}
pub fn main() {
    println!("Quadruple: {:?}", quadruple(2));
}
