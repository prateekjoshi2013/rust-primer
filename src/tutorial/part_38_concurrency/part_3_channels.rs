use std::{sync::mpsc::channel, thread}; // mpsc: multiple producer single consumer

pub fn main() {
    let (tx, rx) = channel();
    let _t_handle = thread::spawn(move || {
        let send_result: () = tx.send(String::from("Sent String")).unwrap_or(());
        /*
        > Attempts to send a value on the channel,
        returning it back if it could not be sent.

        > A successful send occurs when it is determined
        that the other end of the channel has not hung up already.

        > An unsuccessful send would be one where the corresponding
        receiver has already been deallocated.

        > Note that a return value of Err means that the data will never
        be received, but a return value of Ok does not mean that the data
        will be received.

        > This method will never block the current thread.

         */
    });

    /*
    > Attempts to wait for a value on this receiver,
    returning an error if the corresponding channel has hung up.

    > This function will always block the current thread
    if there is no data available

    > It's possible for more data to be sent (at least one sender still exists).
    Once a message is sent to the corresponding Sender (or SyncSender),
    this receiver will wake up and return that message.

    > If the corresponding Sender has disconnected,
    or it disconnects while this call is blocking,
    this call will wake up and return Err to indicate
    that no more messages can ever be received on this channel.

    > However, since channels are buffered, messages sent before
    the disconnect will still be properly received.
     */
    // let received = rx.recv().unwrap_or("No recieved value".to_string());
    // println!("Received from tx: {}", received);
    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is {:?}", received_value);
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }

    _t_handle.join();
}
