#![allow(unused_variables)]
#![allow(unused_mut)]
pub fn main() {
    /*
       Tuples and Arrays
       both have fixed length
       tuples can have different types while arrays have same type
    */
    let my_info = ("Salary", 40_000);
    println!("{} is equal to {}", my_info.0, my_info.1);
    println!("Another way of printing the whole tuple is {:?}", my_info);

    // Destructuring

    let (salary, salary_values) = my_info;
    let nested_tuple = (4, 5.0, (3, 2), "Hello");
    let (num, fl, tup, word) = nested_tuple;
    println!("tuple: {:?}", nested_tuple);

    // empty tuple: this is the default return type of a function
    let empty_tuple = ();

    // Arrays
    // size and type should be known at compile time

    let mut num_array: [i32; 5] = [4, 5, 6, 8, 9];
    println!("{}", num_array[0]);
    println!("{:?}", num_array);
    num_array[0] = 3;

    let mut array_initialized_with_same_elements = [0; 10];
    let mut array_initialized_with_same_elements = ["Unknown"; 10];

    // Array slices
    let mut number_array_1 = [4, 5, 6, 7, 8, 9];
    println!("number array {:?}", number_array_1);
    let subset_array = &mut number_array_1[1..=3];
    subset_array[0] = -5;
    println!(
        "original array after changing mutable slice of number array {:?}",
        number_array_1
    );
    println!("length of array {} ", number_array_1.len());
    println!(
        "the array is occupying {} bytes",
        std::mem::size_of_val(&number_array_1)
    );
    // optinal indexing
    println!(
        "optional indexing using get for valid index: {:?} for invalid index: {:?}",
        number_array_1.get(0),
        number_array_1.get(10)
    )
}
