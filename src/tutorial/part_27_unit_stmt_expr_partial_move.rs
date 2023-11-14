/*
() -> unit type is returned by a
function or block by default which doesnt
return any value

Statements vs Expressions

stmt :
Statements are instructions that perform
some action and do not produce a value.
They end with a semicolon (;)

expr :
Expressions are code fragments that produce
a value when executed. They do not end with
a semicolon.

partial move happens in structs or tuples
when a non primitive value is moved out
rendering the whole structure unusable
but the unmoved values can still be used

*/

fn returns_nothing() {}

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: String,
}

pub fn main() {
    let m = returns_nothing();
    let m = {};
    let m = {
        1 //expression
    }; // statement
    let m = MyStruct {
        a: 21,
        b: String::from("hello world !"),
    };

    // String value moved out of struct causing partial move
    let b = m.b;
    // value copied not moved
    let a = m.a;
    // error because of partial move struct as a whole unusable
    //println!("MyStruct: {:?}", m);
    println!("{}", m.a);
    // error because of partial move struct as a whole unusable
    // println!("{:?}",m.b);
    println!("{:?}", b);
}
