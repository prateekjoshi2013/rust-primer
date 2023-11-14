// we share state in rust using mutex

use std::sync::Mutex;

pub fn main() {
    let m = Mutex::new(5);
    {
        // shared states is accessed only after locking
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    println!("m={:?}", m);

    // This mutex will block threads waiting for the lock to become available
    let mut num1=m.lock().unwrap();
    // this is a deadlock as num1 has acquired lock on mutex 
    // drop(num1); // needed to avoid deadlock
    let mut num2=m.lock().unwrap(); 

}
