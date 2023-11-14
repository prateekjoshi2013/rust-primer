use std::thread::{self, ThreadId};

pub fn main() {
    for _ in 1..=10 {
        let my_string = String::from("My String");
        let join_handle = thread::spawn(move || {
            /*
            > closure may outlive the current function,
            but it borrows `my_string`, which is owned
            by the current function may outlive borrowed
            value `my_string`

            > to force the closure to take ownership of
            `my_string` (and any other referenced variables),
            use the `move`
             */
            println!(
                "Printing moved String: {} from Inside the thread: {:?}",
                my_string,
                thread::current().id()
            );
        });
        join_handle.join();
    }
}
