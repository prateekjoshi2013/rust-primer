// join macro completes all the tasks provided
// to it before proceeding
#[tokio::main]
pub async fn main() {
    /*
    Waits on multiple concurrent branches, returning
    when all branches complete with Ok(_) or on the
    first Err(_).
    */
    let mut results =
        tokio::try_join!(function_1(), function_2()).expect("Failed to join functions");
    println!("results after completion of both tasks: {:?}", results);
}
async fn function_1() -> Result<bool, &'static str> {
    println!("Function 1 has started");
    for _ in 0..100_000_000 {}
    println!("Function 1 has completed");
    Ok(true)
}

async fn function_2() -> Result<bool, &'static str> {
    println!("Function 2 has started");
    for _ in 0..100 {}
    println!("Function 2 has completed");
    Ok(false)
}
