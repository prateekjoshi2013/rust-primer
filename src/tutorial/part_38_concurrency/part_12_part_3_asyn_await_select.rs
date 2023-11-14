#[tokio::main]
pub async fn main() {
    let aw1 = tokio::spawn(async move {
        let res = function_1().await;
        res
    });
    let aw2 = tokio::spawn(async move {
        let res = function_2().await;
        res
    });

    // select macro chooses whichever task completes fist
    // in this case it is function 2 with lesser competion

    // disadvantage of this approach is even though one of
    // two task is selected it waits for both of the tasks
    // to finish
    tokio::select! {
        result=aw1=>println!("function 1 result: {:?}",result),
        result=aw2=>println!("function 2 result: {:?}",result),
    };
}

async fn function_1() -> bool {
    println!("Function 1 has started");
    for _ in 0..100_000_000 {}
    println!("Function 1 has completed");
    true
}

async fn function_2() -> bool {
    println!("Function 2 has started");
    for _ in 0..100 {}
    println!("Function 2 has completed");
    false
}
