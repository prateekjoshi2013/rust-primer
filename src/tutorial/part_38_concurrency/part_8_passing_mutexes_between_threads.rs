use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub fn main() {
    let mut mutex = Arc::new(Mutex::new(0));
    let mut thread_handles = Vec::new();
    for _ in 1..=10 {
        let mutex = Arc::clone(&mutex);
        let join_handle = thread::spawn(move || {
            let mut val = mutex.lock().unwrap();
            *val += 1;
            println!(
                "thread {:?} incrementing to {}",
                thread::current().id(),
                val
            );
        });
        thread_handles.push(join_handle);
    }

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    println!("val after increments: {}", mutex.lock().unwrap());
}
