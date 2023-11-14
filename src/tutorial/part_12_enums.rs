enum Conveyance {
    Car,
    Train,
    Air = 10, // we can assign a value of our
}

impl Conveyance {
    fn travel_allowance(&self, miles: f32) -> f32 {
        match self {
            Conveyance::Car => miles * 1000f32,
            Conveyance::Train => miles * 2000f32,
            Conveyance::Air => miles * 3000f32,
        }
    }
}

// using a tuple struct

enum Value {
    Integer(i32),
    Float(f32),
}

impl Value {
    fn print_val(&self) {
        match self {
            Value::Float(val) => println!("Value::Float({})", val),
            Value::Integer(val) => println!("Value::Integer({})", val),
        }
    }
}

pub fn main() {
    println!(
        "The value of the options are Car: {}, Train: {}, Air:{}",
        Conveyance::Car as usize,
        Conveyance::Train as usize,
        Conveyance::Air as usize,
    );

    println!(
        "Conveyance allowances Car: {} Train: {} Air: {}",
        Conveyance::Car.travel_allowance(60f32),
        Conveyance::Train.travel_allowance(60f32),
        Conveyance::Air.travel_allowance(60f32)
    );

    let value_vectors: Vec<Value> = vec![
        Value::Integer(12),
        Value::Float(12.5f32),
        Value::Integer(13),
    ];

    for val in value_vectors.iter() {
        val.print_val();
    }
}
