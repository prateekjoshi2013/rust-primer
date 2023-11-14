use std::collections::HashMap;

pub fn main() {
    let words = vec![
        "the".to_string(),
        "teh".to_string(),
        "het".to_string(),
        "stupid".to_string(),
        "studpi".to_string(),
        "apple".to_string(),
        "appel".to_string(),
    ];
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for word in words.iter() {
        let mut key = word
            .to_lowercase()
            .chars()
            .into_iter()
            .collect::<Vec<char>>();
        key.sort();
        let key = key.iter().collect::<String>();

        let freq = map.entry(key).or_insert(Vec::new());
        freq.push(word.clone());
    }
    let res: Vec<Vec<String>> = map.into_iter().map(|(_, v)| v).collect();

    println!("for Strings:{:?} => groupings {:?}", words, res);
}
