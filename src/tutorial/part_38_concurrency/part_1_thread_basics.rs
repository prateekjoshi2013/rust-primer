use std::{thread, time::Duration};

pub fn main() {
    println!("This will be printed");
    println!("This will also be printed");
    println!("The concurrency will start after this line");
    let t = thread::spawn(|| {
        println!("Hello 1 from the thread : {:?}", thread::current().id());
        println!("Hello 2 from the thread : {:?}", thread::current().id());
        println!("Hello 3 from the thread : {:?}", thread::current().id());
        println!("Hello 4 from the thread : {:?}", thread::current().id());
        println!("Hello 5 from the thread : {:?}", thread::current().id());
        println!("Hello 6 from the thread : {:?}", thread::current().id());
        println!("Hello 7 from the thread : {:?}", thread::current().id());
    });
    // makes the main thread sleep for 1 millis
    // thread::sleep(Duration::from_millis(1));
    t.join(); // makes the main thread wait untill the spawned thread complete
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
}
