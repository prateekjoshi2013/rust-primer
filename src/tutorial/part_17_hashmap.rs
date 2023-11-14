use std::collections::HashMap;

pub fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("Prateek", 40);
    person.insert("Ayush", 45);
    person.insert("Mital", 43);

    // getting value
    println!("The age is {:?}", person.get("Prateek").unwrap());
    if person.contains_key("Ayush") {
        println!("Contains Ayush");
    } else {
        println!("Does Not Contains");
    }

    //iterating on map as tuple (key value)

    for (name, age) in person.iter() {
        println!("my name is {} and my age is {}", name, age);
    }

    // insert if entry is not present
    person.entry("Pratima").or_insert(42);
    person.entry("Pratima").or_insert(39);
    println!("Map: {:?}", person);

    // remove key
    if let Some(marks) = person.remove("Prateek") {
        println!("removed Prateek from map with value: {}", marks);
    }

    // insert and update
    let mut map: HashMap<i32, i32> = HashMap::new();
    for v in vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5] {
        let freq = map.entry(v).or_insert(0);
        *freq += 1;
    }
    println!("Frequency Map: {:?}", map);
}
