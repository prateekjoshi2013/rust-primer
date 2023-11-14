/*
Question mark operator is used in relation with
    Result{
        Ok,Err
    }
enum
*/
#[derive(Debug)]
enum MathError {
    DvisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

type MathResult = Result<(), MathError>;

fn division(n: f64, d: f64) -> MathResult {
    match d {
        0.0 => Err(MathError::DvisionByZero),
        _ => {
            println!("{}/{}={}", n, d, n / d);
            Ok(())
        }
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        println!("sqrt({})=>{}", x, x.sqrt());
        Ok(())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
    } else {
        println!("ln({})=>{}", x, x.ln());
        Ok(())
    }
}

fn calculate_ok() -> MathResult {
    division(5.0f64, 2.0f64)?;
    sqrt(64f64)?;
    ln(5.5f64)?;
    Ok(())
}

fn calculate_err() -> MathResult {
    division(5.0f64, 0.0f64)?;
    sqrt(64f64)?;
    ln(5.5f64)?;
    Ok(())
}
pub fn main() {
    println!("Math Result: {:?}", calculate_ok());
    println!("Math Result: {:?}", calculate_err());
}
