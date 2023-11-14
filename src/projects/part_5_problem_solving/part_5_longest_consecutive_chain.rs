use std::{collections::HashSet, usize::MIN};

pub fn main() {
    let mut max_len = MIN;
    let vec1=vec![3, 1, 2, 5, 7, 10, 11, 14];
    let vec2=vec![3, 1, 15, 5, 13, 12, 10, 14, 15, 16, 17, 18, 8, 9];
    let vec3=vec![4, 1, 3, 2, 5, 6, 8, 10, 11];
    let mut set: HashSet<i32> = vec1.into_iter().collect();
    let v: Vec<i32> = set.clone().into_iter().collect();

    for i in v.iter() {
        println!("{:?},{:?}", i, set);
        let mut n = *i;
        if !set.contains(&n) {
            continue;
        }
        set.remove(&n);
        let mut size = 1;
        n = *i + 1;
        while set.contains(&n) {
            set.remove(&n);
            size += 1;
            n += 1;
        }
        let mut n = *i - 1;
        while set.contains(&n) {
            set.remove(&n);
            size += 1;
            n -= 1;
        }
        if max_len < size {
            max_len = size;
        }
    }

    println!("max_len:{}", max_len);
}
