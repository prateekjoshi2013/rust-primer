pub fn main() {
    let (first_number, second_number) = (250, 480.22);
    let large_number = 1_0000_000;
    println!(
        "first_number={} , second_number={}, large_number={}",
        first_number, second_number, large_number
    );
    let x = 255;
    println!(
        "The value of x in octal : {:o} , hexadecimal : {:x}, binary : {:b}",
        x, x, x
    );
    let n1 = 14i32;
    let n2 = 2.5f32;
    let n3 = n1 as f32 + n2;
    println!("{}", n3);

    /*
       Shadowing
       1) variable value
       2) variable type
       3) variable mutatbility
       4) block scoped
    */

    let n1 = 54;
    let n2 = true;
    println!("n2={}", n2);
    let mut n2 = 2.7;
    println!("n2={}", n2);
    n2 = 3.5f64;
    {
        let n2 = false;
        println!("n2={}", n2);
    }
    println!("n1={} n2={} n3={}", n1, n2, n3);

    /*
    constants
       - types are not inferred by compiler
       - all upper case with underscores
    */

    const MAX_SALARY: u32 = 100_000;
    println!("MAX_SALARY={}", MAX_SALARY);
}
