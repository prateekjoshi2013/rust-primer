fn dangling_references_example() {
    let i: &i32;
    {
        let j = 5;
        /*

        i = &j;

        `j` does not live long enough
        borrowed value does not live long enough
        */
    }
    // println!("the value of i = {}", i);
    let some_int = 10;
    // let additional_int = some_fn(some_int);
    let additional_int = some_fn(&some_int);
    println!("{}", additional_int);
}

/*
    fn some_fn(i: i32) -> &i32 {
        &i
        // i is no more available so it returns a dangling pointer
    }

*/

fn some_fn(i: &i32) -> &i32 {
    &i
    // i is available as it is ref and returns same ref
}

pub fn main() {
    /*
    lifetime specifiers prevent:
    -  We need lifetime specifier in cases of dangling references
    -  We need lifetime specifier in cases of indeterminate references

    We need lifetime specifiers only when
    -  references are output of some function
    -  the type of struct field is a reference type
    */
    dangling_references_example();
    indeterminate_references_example();
}

fn indeterminate_references_example() {

    /*
        the compiler does not know the which reference
        and hence which ref's lifetime is valid

    fn greater(i: &i32, j: &i32) -> &i32 {
        if i > j {
            i
        } else {
            j
        }
    }

    fn greater(i: &i32, j: &i32) -> &i32 {
        if i > j {
            i
        } else {
            j
        }
    }
    */
}

// This function
fn greater<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j {
        i
    } else {
        j
    }
}

#[derive(Debug)]
struct Person<'a> {
    // required to ensure name str reference
    //stays more than struct instance
    name: &'a str,
    age: i32,
}

fn struct_with_references() {
    let first_name = "Prateek";
    let mut person = Person {
        name: first_name,
        age: 40,
    };
    {
        let last_name = String::from("Azam");
        /*
        this is going to be compiler error becaue last_name
        variable goes out of scope after this block ends
        but person struct instance is being referenced outside
        of this block

        person.name = &last_name;
        */
    }
    println!("Person: {:?}", person);
}
