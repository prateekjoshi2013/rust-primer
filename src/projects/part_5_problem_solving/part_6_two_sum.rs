use std::collections::HashSet;

pub fn main() {
    let products = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];
    let mut set: HashSet<i32> = HashSet::new();
    let mut res: Vec<Vec<i32>> = Vec::new();
    for product in &products {
        if set.contains(&(50 - *product)) {
            res.push(vec![(50 - *product), *product]);
        } else {
            set.insert(*product);
        }
    }

    println!("groups of recommended items : {:?}", res);
}
