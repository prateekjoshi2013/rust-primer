use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    root: Option<Node>,
}

#[derive(Debug)]
struct Node {
    children: HashMap<char, Node>,
    is_leaf: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            children: HashMap::new(),
            is_leaf: false,
        }
    }
}

impl Trie {
    fn new() -> Trie {
        Trie { root: None }
    }

    fn insert(&mut self, word: &str) -> &mut Self {
        let mut curr = self.root.get_or_insert(Node::new());
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(Node::new());
        }
        curr.is_leaf = true;
        self
    }

    fn search(&mut self, word: &str) -> Vec<String> {
        let mut temp_word = String::new();
        if self.root.is_none() {
            return vec![temp_word];
        }
        let mut curr = self.root.as_ref().unwrap();
        for c in word.chars() {
            if curr.is_leaf {
                return vec![temp_word];
            }
            temp_word.push(c);
            curr = curr.children.get(&c).unwrap();
        }
        let mut res = Vec::<String>::new();
        for word in Trie::collect_words(curr) {
            let mut new_temp_word = temp_word.clone();
            new_temp_word.push_str(&word);
            res.push(new_temp_word);
        }
        res
    }

    fn collect_words(curr: &Node) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if curr.is_leaf {
            res.push("".to_string());
        }
        for (&c, node) in &curr.children {
            let temp_res = Trie::collect_words(node);
            for new_word in temp_res {
                res.push(format!("{}{}", c, new_word));
            }
        }
        res
    }
}

pub fn main() {
    let mut trie = Trie::new();
    trie.insert("the")
        .insert("is")
        .insert("isotope")
        .insert("isolate")
        .insert("any")
        .insert("thief");
    println!("{:?}", trie.search("a"));
}
