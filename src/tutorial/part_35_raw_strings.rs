pub fn main() {
    let str = "my name is \n\t\"Prateek\"";
    println!("{}", str);
    /*
    Normal String
    Output:
    my name is
        "Prateek"
     */

    let str = r#"my name is \n\t"Prateek""#;
    println!("{}", str);
    let str = r##"my name is "#\n\t"Prateek""##;
    println!("{}", str);
    /*
    Output:
    my name is \n\t"Prateek"

    > Raw String is used when we dont want to escape a character
    > We need to include # when we have " in the string
    > If we have a # in the raw string we add an extra #" on each
    ends

    */
}
