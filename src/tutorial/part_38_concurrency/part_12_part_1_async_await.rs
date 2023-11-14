// we cannot await in a non async scope
// the top level future needs an async runtime
// this is provided by third party library tokio

// async codes are lighter an no cost abstraction
// to threading
async fn printing() {
    println!("I am async function");
}

#[tokio::main]
pub async fn main() {
    let x = printing();
    println!("The await function has not been called yet");
    x.await;
    // drop(x) will not be awaited if future is dropped
}
