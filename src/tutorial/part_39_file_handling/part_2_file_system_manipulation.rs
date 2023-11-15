use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn main() {
    let path = Path::new(r"~/dev/dev/rust/tutorials/learning_rust/my_text.txt");
    println!("Folder containing the file: {:?}", path.parent().unwrap());

    println!("Name of the file is: {:?}", path.file_stem().unwrap());
    println!("Extension of the file is: {:?}", path.extension().unwrap());

    // creating a file

    let mut path = PathBuf::from(r"~/dev/dev/rust/tutorials");
    path.push("Rust");
    path.push("Examples");
    path.push("my_file");

    path.set_extension("txt");

    println!("the path is: {:?}", path);

    let path = [
        r"~/dev/dev/rust/tutorials",
        "Rust",
        "Examples",
        "my_file.txt",
    ]
    .iter()
    .collect::<PathBuf>();
    println!("the path is: {:?}", path);

    let path = [
        r"/home/dev/dev/rust/tutorials",
        "Rust",
        "Examples",
        "my_file.txt",
    ]
    .iter()
    .collect::<PathBuf>();

    let path = Path::new(r"/home/dev/dev/rust/tutorials/learning_rust/my_text.txt");
    // to check if a path is a directory
    println!("Is the path directory {:?}", path.is_dir());

    // to check if a path is a file
    println!("Is the path file {:?}", path.is_file());

    // metadata of a file
    let path_metadata = path.metadata().unwrap();
    println!("File length : {:?}", path_metadata.len());
    println!("File type : {:?}", path_metadata.file_type());
    println!("File permissions : {:?}", path_metadata.permissions());

    // list files in a directory
    let path = Path::new(r"./");
    for file in path.read_dir().expect("read_dir call failed") {
        println!("{:?}", file.unwrap().path());
    }

    // find current dir path
    let mut curr = env::current_dir().expect("Can't access current directory");
    println!("{:?}", curr);

    // create dir

    println!(
        "Create a new directory: {:?}",
        fs::create_dir(r"/home/dev/dev/rust/fs")
    );

    println!(
        "Create a new directory: {:?}",
        fs::create_dir_all(r"/home/dev/dev/rust/fs/fs1/fs2")
    ); // creates all parent dir if not present

    // removes an empty directory
    println!(
        "Remove a specific directory: {:?}",
        fs::remove_dir(r"/home/dev/dev/rust/fs/fs1/fs2")
    );

    // force removes a directory
    println!(
        "Remove a specific directory: {:?}",
        fs::remove_dir_all(r"/home/dev/dev/rust/fs")
    );

    // renaming a file
    println!(
        "Rename a specific file: {:?}",
        fs::rename(r"./my_text.txt", r"./my_text_1.txt")
    );

    // renaming a file
    println!(
        "Copying contents from a specific file to another: {:?}",
        fs::copy(r"./my_text_1.txt", r"./my_text_2.txt")
    );

    // remove a file
    println!(
        "Remove a specific file: {:?}",
        fs::remove_file(r"./my_text_2.txt")
    );
}
