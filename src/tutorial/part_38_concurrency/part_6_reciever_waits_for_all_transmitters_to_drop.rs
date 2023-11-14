use std::{
    sync::mpsc,
    thread::{self, ThreadId},
};

fn timer(d: i32, tx: mpsc::Sender<(ThreadId, i32)>) {
    thread::spawn(move || {
        println!("sent: {}  by thread: {:?}", d, thread::current().id());
        tx.send((thread::current().id(), d)).unwrap();
    });
}
pub fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        timer(i, tx.clone());
    }

    drop(tx);
    for recv in rx {
        println!("received {} from {:?}", recv.1, recv.0);
    }
    // since tx has is still not dropped rx is blocked waiting on tx
    // we only passed clones of tx to all the threads so if we drop tx before
    // loop on rx theprogram will finish
}
