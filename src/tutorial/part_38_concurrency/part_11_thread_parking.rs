use std::{thread, time::Duration};

pub fn main() {
    let job_1 = thread::spawn(|| {
        println!("---Job 1 has started---");
        println!("Waiting for job 2 to complete");
        // this pauses the thread until its unparked
        thread::park();
        // thread pauses until timeout or when its unparked
        // thread::park_timeout(Duration::from_secs(2));
        // this gives out its time back to os so other
        // threads can be scheduled
        // thread::yield_now();
        println!("---Job 1 has resumed---");
        println!("---Job 1 has finished---");
    });

    let job_2 = thread::spawn(|| {
        println!("---Job 2 has started---");
        println!("---Job 2 has finished---");
    });
    job_2.join().unwrap();
    println!("Job 2 is now completed");
    println!("Job 1 will now resume");
    job_1.thread().unpark();
    job_1.join().unwrap();
}
