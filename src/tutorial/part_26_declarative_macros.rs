/* Declarative Macros */

/*
Capture types capture the parameters for pmatching cases

    expression - expr,

    expr stands for "expression" and is used when you want
    to match and work with any valid Rust expression.
    If your macro needs to manipulate or evaluate expressions,
    you would use expr

    type - ty,

    ty stands for "type" and is used when you want to match any
    valid Rust type. If your macro needs to work with types, you
    would use ty

    identifier - ident

    ident stands for "identifier" and is used for variable names,
    function names, struct names, and similar constructs.
    When you want to match any valid Rust identifier in your macro,
    you can use ident
*/
macro_rules! our_macro {
    () => {
        // matches empty param list
        println!("my macro")
    };
    (execute my macro) => {
        1 + 1
    };
    ($e1:expr ,$e2:expr, $e3:expr) => {
        $e1 + $e2 + $e3
    };
    ($e1:expr ;$e2:expr; $e3:expr) => {
        $e1 * $e2 * $e3
    };
}

macro_rules! input {
    ($t: ty) => {{
        let mut m = String::new();
        std::io::stdin()
            .read_line(&mut m)
            .expect("failed to read input");
        let n: $t = m.trim().parse().expect("invalid input");
        n
    }};
}

macro_rules! add_as {
    ($a: expr, $b:expr , $typ:ty ) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! some_macro {
    ($var: ident) => {
        $var = $var + 1;
    };
}

macro_rules! create_function {
    ($func_name:ident,$input: ident,$type_input: ty,$type_output:ty) => {
        fn $func_name($input: $type_input) -> $type_output {
            println!(
                "you called {:?}() with input of {:?}",
                stringify!($func_name),
                stringify!($input)
            );
            $input
        }
    };
}

/* 
    Repeating Pattern Macros to match repeating inputs to macro 
    * repeat 0 or more times
    + repeat 1 or more times
    ? repeat 0 or 1 times
*/

macro_rules! repeating_concat {
    ($($str:expr), *) => {
       {
            let mut res=String::new();
            $(res.push_str($str);)*
            res
        }
    };
}

pub fn main() {
    our_macro!();
    println!(
        "our_macro(execute my macro)={}",
        our_macro!(execute my macro)
    );

    println!("our_macro(2,3,4)={}", our_macro!(2, 3, 4));
    println!("our_macro(2,3,4)={}", our_macro!(2; 3; 4));

    let string_input = input!(String);
    println!("my input: {}", string_input);
    println!("my addition: {}", add_as!(15, 2.3, f32));
    let mut x = 1;
    // since identifiers or variables are not
    // transferred in and out of macro we use ident
    // type to pass them in and out of macro boundary
    some_macro!(x);

    /*
    macro ownership rules

    macros do not change ownership as long as we
    dont do it in the expanded code

    */
    f1(15);

    let str_null = repeating_concat!();
    println!("{}", str_null);
    let str_repeat_1 = repeating_concat!("first");
    println!("{}", str_repeat_1);
    let str_repeat_2 = repeating_concat!("first", "second");
    println!("{}", str_repeat_2);
}

create_function!(f1, x, i32, i32);
