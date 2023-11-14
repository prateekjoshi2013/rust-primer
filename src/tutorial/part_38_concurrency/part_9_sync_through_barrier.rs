use std::{
    sync::{Arc, Barrier, Mutex},
    thread,
};

pub fn main() {
    let data = vec![
        vec![1, 2, 3, 4, 5, 6],
        vec![11, 12, 13, 14, 15, 16],
        vec![21, 22, 23, 24, 25, 26],
    ];
    let mut join_handles = Vec::new();
    let data = Arc::new(data);
    let barrier = Arc::new(Barrier::new(3));
    let result = Arc::new(Mutex::new(0));
    for i in 0..data.len() {
        let data = Arc::clone(&data);
        let result = Arc::clone(&result);
        let barrier = Arc::clone(&barrier);
        let join_handle: thread::JoinHandle<()> = thread::spawn(move || {
            let sum: i32 = data[i][0..3].iter().sum();
            let mut total_sum = result.lock().unwrap();
            *total_sum += sum;
            // needed to release lock on total_sum
            // the lock on total_sum exists until total_sum is not dropped
            drop(total_sum);
            // All threads wait until the 3 of them reach this point
            println!(
                "Thread {:?} reached partial sum {}",
                thread::current().id(),
                result.lock().unwrap()
            );
            barrier.wait();
            let sum: i32 = data[i][3..].iter().sum();
            let mut total_sum = result.lock().unwrap();
            *total_sum += sum;
            // needed to release lock on total_sum
            // the lock on total_sum exists until total_sum is not dropped
            drop(total_sum);
            println!(
                "Thread {:?} Completed processing total_sum :{}",
                thread::current().id(),
                result.lock().unwrap()
            );
        });
        join_handles.push(join_handle);
    }

    for join_handle in join_handles {
        join_handle.join().unwrap();
    }
    println!("The result of the sum is {:?}", result);
}
