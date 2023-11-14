pub fn main() {
    let mut vec1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    for i in 0..vec1.len() {
        println!("vec1[{}]={}", i, vec1[i]);
    }
    // i is borrowed as immutable ref of vector elements
    for i in vec1.iter() {
        println!("{}", i);
    }

    // equivqlent fo loop

    for i in &vec1 {
        println!("{}", i);
    }

    // i is borrowed as mutable ref of vector elements
    // which can change value of the element in vector
    for i in vec1.iter_mut() {
        *i += 1; // * is called deref
    }

    // equivalent for loop

    for i in &mut vec1 {
        *i += 1; // * is called deref
    }

    println!("modified error : {:?}", vec1);

    // vector is consumed and all the values are moved
    // here vec1.into_iter() is called implicitly
    
    for i in vec1/*.into_iter()*/{
        println!("{}", i);
    }
    // the value is moved because of call to into_iter by for loop
    // println!("modified error : {:?}", vec1);
}
