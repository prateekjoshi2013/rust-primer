use std::{cmp::Ordering, collections::BinaryHeap};

struct Node {
    val: i32,
    row: usize,
    col: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Implement your custom comparison logic based on the `value` field
        other.val.cmp(&self.val).reverse() // Order nodes by value in descending order
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Node {}

pub fn main() {
    let q = vec![
        vec![9, 12, 18, 25, 90],
        vec![5, 13, 19],
        vec![4, 10, 40, 45],
    ];
    let mut res = Vec::<i32>::new();
    let mut pq = BinaryHeap::<Node>::with_capacity(5);
    for i in 0..q.len() {
        let val = &q[i][0];
        pq.push(Node {
            val: *val,
            row: i,
            col: 0,
        });
    }

    while !pq.is_empty() {
        match pq.pop() {
            Some(node) => {
                res.push(node.val);
                if node.col+1 < q[node.row].len() {
                    pq.push(Node {
                        val: q[node.row][node.col + 1],
                        col: node.col + 1,
                        row: node.row,
                    });
                }
            }
            None => break,
        }
    }
    println!("{:?}", res);
}
