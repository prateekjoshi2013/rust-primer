use std::{sync::mpsc, thread, time::Duration};

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    let join_handle_thread_1 = thread::spawn(move || {
        let values = vec![
            (thread::current().id(), 1),
            (thread::current().id(), 2),
            (thread::current().id(), 3),
            (thread::current().id(), 4),
            (thread::current().id(), 5),
            (thread::current().id(), 6),
        ];
        for value in values {
            tx.send(value).unwrap();
        }
        // makes thread sleep so other thread can takover
        thread::sleep(Duration::from_nanos(1));
    });
    let join_handle_thread_2 = thread::spawn(move || {
        let values = vec![
            (thread::current().id(), 1),
            (thread::current().id(), 2),
            (thread::current().id(), 3),
            (thread::current().id(), 4),
            (thread::current().id(), 5),
            (thread::current().id(), 6),
        ];
        for value in values {
            tx_clone.send(value).unwrap();
        }
        // makes thread sleep so other thread can takover
        thread::sleep(Duration::from_micros(1));
    });

    for recv_value in rx {
        println!(
            "value: {} recieved from Thread: {:?}",
            recv_value.1, recv_value.0
        );
        // makes thread sleep so other thread can takover
        thread::sleep(Duration::from_micros(1));
    }
    join_handle_thread_2.join();
    join_handle_thread_1.join();
}
