pub fn main() {
    let m = Some(String::from("Hello World!"));
    match m {
        Some(moved_string) => println!("{}", moved_string),
        None => (),
    }
    // partial move happens of String value
    // from Option m
    // to moved_string
    // println!("{:?}",m); error

    // to avoid this we can convert the contains
    // value in option to reference of the contained
    // value

    let m = Some(String::from("Hello World!"));
    match &m {
        Some(moved_string) => println!("{}", moved_string),
        None => (),
    }
    println!("{:?}", m);

    // another way is

    let m = Some(String::from("Hello World!"));
    match m.as_ref() {
        Some(moved_string) => println!("{}", moved_string),
        None => (),
    }

    println!("{:?}", m);
}
