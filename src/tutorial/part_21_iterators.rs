// iter() and iter_mut() 
// > Do not take ownership and borrows 
// immutable and mutably respectively 
// > It needs Iterator trait to be implemented

// into_iter() 
// > Consumes the target object takes ownership
// > Needs IntoIterator trait to be implemented 

pub fn main() {
    let some_vec = vec![1, 2, 3, 4, 5, 6, 7];
    let mut iter = some_vec.iter();
    println!("The iterator: {:?}", iter);
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter); // iterator is consumed here hence empty

    /*------------- operations on iterator--------------*/

    /* Any */

    // &x basically allows copy of value x to x variable in closure as i32 implements copy trait

    println!("{}", 
    some_vec.iter()
    /* &x copies the value of the argument which is of &i32 type to x as i32 implements Copy trait*/
    .any(|&x| x > 0));

    println!("{}",
    vec!["abc".to_string(), "".to_string(), "dasd".to_string()]
        .iter()
        /*cannot use &x here as the argument passed to any is &String and String does not implement Copy like i32 above */
        .any(|x| x.len() > 0));

    println!("{}", some_vec.iter().all(|&x| x > 0));

    println!(
        "{}",
        vec!["abc".to_string(), "".to_string(), "dasd".to_string()]
            .iter()
            .all(|x| x.len() > 0)
    );

    /* find */

    let string_vec = vec!["abc".to_string(), "".to_string(), "dasd".to_string()];
    let found = string_vec.iter().find(|&x| x.eq("abc"));
    println!("{:?}", found.unwrap());

    let found = some_vec
        .iter()
        //find gets a double reference hence we are using &&x to
        //copy value behind double reference to arg x
        .find(|&&x| x == 2);
    println!("{:?}", found.unwrap());

    /* first position */

    let position: Option<usize> = some_vec.iter().position(|&x| x > 2);
    println!("position: {}", position.unwrap());

    let position = string_vec.iter().position(|x| x.eq("abc"));
    println!("position: {}", position.unwrap());

    /* first index position from right */

    let rposition: Option<usize> = some_vec.iter().rposition(|&x| x == 2);
    println!("rposition: {}", rposition.unwrap());

    let rposition = string_vec.iter().rposition(|x| x.eq("abc"));
    println!("rposition: {}", rposition.unwrap());

    /* max, max_by */

    let max = string_vec.iter().max_by(|&x, &y| x.cmp(y));
    println!("max: {}", max.unwrap());

    let max = some_vec.iter().max();
    println!("max: {}", max.unwrap());

    /* min, min_by */

    let max = string_vec.iter().min_by(|&x, &y| x.cmp(y));
    println!("min: {}", max.unwrap());

    let max = some_vec.iter().min();
    println!("min: {}", max.unwrap());

    // reversed iterator

    for x in string_vec.iter().rev() {
        print!("{} ", x);
    }
    println!();
    for x in some_vec.iter().rev() {
        print!("{} ", x);
    }

    println!();

    // --------------filter, collect function----------

    let filtered_vector = some_vec
        // iter() returns &i32
        .iter()
        .filter(|&&x| x % 2 == 0)
        .collect::<Vec<&i32>>();

    println!("filtered vector => {:?}", filtered_vector);
    let cloned_vector=some_vec
    .clone();
    let filtered_vector = 
        // into_iter() returns i32 and the
        // value is moved out of original vector
        cloned_vector.into_iter()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<i32>>();

    println!("filtered vector => {:?}", filtered_vector);

    // --------------filter, map, collect function----------

    let filtered_mapped_vector = some_vec
        // iter() returns &i32
        .iter()
        .filter(|&&x| x % 2 == 0)
        /*
         It produces a new iterator which calls this closure 
         on each element of the original iterator
         */
        .map(|&x|x*2)
        .collect::<Vec<i32>>();

    println!("filtered mapped vector => {:?}", filtered_mapped_vector);

    let cloned_vector=some_vec
    .clone();
    let filtered_mapped_vector = 
        // into_iter() returns i32 and the
        // value is moved out of original vector
        cloned_vector.into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x|x*2)
        .collect::<Vec<i32>>();

        println!("filtered mapped vector => {:?}", filtered_mapped_vector);

}
