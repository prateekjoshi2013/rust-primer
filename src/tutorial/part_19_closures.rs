pub fn main() {
    let x = 5;
    let square = |x: i32| println!("The square of the variable is: {}", x * x);
    square(x); // since the value is non-primitive it is not moved
               /*
                   Unlike functions which only has access to parameters
                   and variables declared inside functions

                   closures capture all the variables in the code block in which the closure
                   is defined

               */
    let square = || println!("The square of the variable is: {}", x * x);
    square();

    let print_info = |general_info: String, name: &str, age: i32| {
        println!(
            "general_info: {}, name: {}, age:{}",
            general_info, name, age
        )
    };

    let (general_info, person_age, person_name) =
        (String::from("The details are"), 51, String::from("Prateek"));
    print_info(
        general_info, // String type value moved to closure
        &person_name, // borrowed by reference
        person_age,   // i32 copied by value
    );
    // println!("{:?}",general_info) borrow of moved value: `general_info`
    println!("general_info: {}, person_age: {}", person_name, person_age);

    let square = |num| num * num;
    println!("x: {} Squared: {}", 5, square(5)); // type i32 bound to untyped closure
                                                 // square(5.0) wont work now since it was called first time with i32

    // A closure can be returned or passed to and from a function
    println!(
        "division: {}/{} => {:?}",
        5,
        3,
        division(5.0, 3.0, |x: f32| x != 0.0)
    );

    println!(
        "division: {}/{} => {:?}",
        5,
        0.0,
        division(5.0, 0.0, |x: f32| x != 0.0)
    );

    /*--------------------------------------------- */
    println!("--------------------------case1------------------- ");
    let mut vec1 = vec![1, 2, 3];
    let some_closure = || {
        // since vec1 is not mutated here it is captured
        // as an immutable reference here by compiler at runtime
        println!("inside closure Vec 1: {:?}", vec1);
    };

    println!("outside closure Vec 1: {:?}", vec1);
    some_closure();

    /*--------------------------------------------- */

    println!("--------------------------case2------------------- ");
    let mut vec1 = vec![1, 2, 3];
    // closure needs to be caputred as mutable
    // because vector is being modified inside
    let mut some_closure = || {
        // since vec1 is mutated here it is captured
        // as a mutable reference here by compiler at runtime
        vec1.push(5);
        println!("inside closure Vec 1: {:?}", vec1);
    };

    // since lifetime of mutable ref to vec
    // ends after this call inside closure
    // vec1 can be modified after the call
    some_closure();
    vec1.push(20);
    println!("outside closure Vec 1: {:?}", vec1);
}

fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) -> Option<f32> {
    if f(y) {
        Some(x / y)
    } else {
        None
    }
}
