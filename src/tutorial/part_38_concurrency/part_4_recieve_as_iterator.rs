use std::{sync::mpsc, thread};

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
        }
    });

    for recv_val in rx {
        println!("The received values are {:?}", recv_val);
    }

    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
        }
    });

    let recv_values = rx.iter().collect::<Vec<i32>>();
    println!("the received values are: {:?}", recv_values);
}
