pub fn main() {
    let x: f32 = 15f32;
    println!("the value of variable x = {}", x);
    // x = 60.0 will fail because by default immutable

    let mut x: u32 = 8;
    println!("the value of variable x = {}", x);

    /*
       Variable naming:
       - Only letters, digits, underscores
       - begin with letter or underscore
       - Case sensitive
    */
    x = 5 * 5;
    println!("{}", x);
    /*
       integer
           - signed : i8 i16 i32(default) i64
           - unsigned: u8 u16 u32 u64
    */

    println!("the maximum in i8 is = {}", std::i8::MAX);
    println!("the maximum in u8 is = {}", std::u8::MAX);

    /*
        floats
            - f32
            - f64(default)
    */

    println!("the maximum in i8 is = {}", std::f32::MAX);
    println!("the maximum in u8 is = {}", std::f64::MAX);

    // Automatic type casting is not allowed for ex float + int is not valid

    /*
       Bool
           - true
           - false
    */
    let status: bool = true;
    println!("{}", status);

    /*
       char
    */
    let c1: char = 'a';
    let c2: char = 'b';
    println!("characters {} {}",c1,c2);
}
