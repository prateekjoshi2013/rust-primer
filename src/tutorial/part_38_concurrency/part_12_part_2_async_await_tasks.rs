#[tokio::main]
pub async fn main() {
    let mut handles = vec![];
    println!("This code is not part of the async block");
    let s1 = String::from("Huge Computation function");
    let s2 = String::from("Simpler Computation function");
    // async task 1
    let aw1 = tokio::spawn(async move {
        huge_computation(s1).await;
    });
    // async task 2
    let aw2 = tokio::spawn(async move {
        simpler_computation(s2).await;
    });
    handles.push(aw1);
    handles.push(aw2);

    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks are now completed!!!");
}

async fn huge_computation(s: String) {
    println!("{:?} has started", s);
    for i in 0..100_000_000 {}
    println!("{:?} is now completed", s);
}

async fn simpler_computation(s: String) {
    println!("{:?} has started", s);
    println!("{:?} is now completed", s);
}
