use std::fs::*; // Filesystem manipulation operations.
use std::io::{BufRead, BufReader, Read, Write}; // for manipulating contents of a file
use std::path::Path; // for manipulating file paths

// returns a specialized [Result] type for I/O operations.
fn write_to_file() -> std::io::Result<()> {
    let path_loc = r"/home/dev/dev/rust/tutorials/learning_rust/my_text.txt";
    let path = Path::new(path_loc);
    // this opens the file in overwrite mode by default
    let mut file = File::create(path)?;
    /*
    file.write(b"lets put this in the file")?;
    // use b"" or as_bytes to convert strings to [u8]
    file.write("lets put this in the file as well".as_bytes())?;
     */
    // to open file in append mode
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write("\n www.includehelp.com\n".as_bytes())?;
    let some_vec = vec![1, 2, 3, 4, 5];
    let vec_to_string = some_vec
        .into_iter()
        .map(|a| format!("{},", a.to_string()))
        .collect::<String>();
    file.write(vec_to_string.as_bytes())?;
    let (name, age) = ("Joseph", 40);
    let formatted_str = format!("I am {} and my name is {}", name, age);
    file.write(formatted_str.as_bytes())?;
    Ok(())
}

fn read_file_line_by_line() -> std::io::Result<()> {
    let path_loc = r"./my_text.txt";
    let file = File::open(path_loc)?;
    let file_buffer = BufReader::new(file);
    for lines in file_buffer.lines() {
        println!("{:?}", lines?);
    }

    Ok(())
}

fn read_file() -> std::io::Result<()> {
    let path_loc = r"./my_text.txt";
    let mut file = File::open(path_loc)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(())
}

pub fn main() {
    println!("{:?}", write_to_file());
    println!("{:?}", read_file());
    println!("{:?}", read_file_line_by_line());
}
