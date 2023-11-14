use std::collections::HashMap;

pub fn main() {
    let mut product_map = HashMap::<&str, Vec<u8>>::new();
    product_map.insert("product_a", vec![1, 2, 2, 3]);
    product_map.insert("product_b", vec![4, 5, 6, 3, 4]);
    product_map.insert("product_c", vec![8, 8, 7, 6, 5, 4, 4, 1]);

    let mut popularity_map: HashMap<&str, Vec<&str>> = HashMap::new();
    popularity_map.insert("increasing", Vec::new());
    popularity_map.insert("decreasing", Vec::new());
    popularity_map.insert("fluctuating", Vec::new());

    for (product, pop_scores) in &product_map {
        let mut pop = "";
        for i in 1..pop_scores.len() {
            if pop_scores[i - 1] == pop_scores[i] {
                continue;
            } else if pop_scores[i - 1] < pop_scores[i] && (pop.is_empty() || pop.eq("increasing"))
            {
                pop = "increasing";
            } else if pop_scores[i - 1] > pop_scores[i] && (pop.is_empty() || pop.eq("decreasing"))
            {
                pop = "decreasing";
            } else {
                pop = "fluctuating"
            }
        }
        popularity_map
            .entry(pop)
            .or_insert(Vec::new())
            .push(*product);
    }

    println!("{:?}", popularity_map);
}
