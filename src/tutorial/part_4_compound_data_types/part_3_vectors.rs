pub fn main() {
    /*
       Vectors
        - resizable arrays
        - same type
        - grow and shrink at run time
    */
    let mut number_vec: Vec<i32> = vec![4, 5, 6, 7, 8, 9];
    println!("{}", number_vec[0]);
    println!("{:?}", number_vec);
    number_vec[4] = 5;
    println!("{:?}", number_vec);
    let _vector_with_same_elements: Vec<i32> = vec![0; 10];
    let subset_vec: &[i32] = &number_vec[0..4];
    println!("slice of vector: {:?}", subset_vec);

    // functions on vectors

    println!("length of a vector {}", number_vec.len());
    println!("optional indexing of a vector {:?}", number_vec.get(100));
    number_vec.push(10);
    println!("pushing 10 in a vector {:?}", number_vec);
    number_vec.pop();
    println!("popping 10 on a vector {:?}", number_vec);
    number_vec.remove(0);
    println!("removing on index 0 on a vector {:?}", number_vec);
    println!(
        "does value 10 exists in vector ? : {}",
        number_vec.contains(&10)
    );
}
