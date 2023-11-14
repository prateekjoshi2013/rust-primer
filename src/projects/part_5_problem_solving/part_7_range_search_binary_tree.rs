#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, val: i32) -> &mut Tree {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(val)));
        } else {
            let mut curr = self.root.as_mut().take();
            while curr.is_some() {
                curr = curr
                    .map(|node| {
                        if val < node.val && node.left.is_some() {
                            node.left.as_mut()
                        } else if val >= node.val && node.right.is_some() {
                            node.right.as_mut()
                        } else {
                            if val < node.val {
                                node.left = Some(Box::new(Node::new(val)));
                            } else {
                                node.right = Some(Box::new(Node::new(val)));
                            }
                            None
                        }
                    })
                    .unwrap();
            }
        }
        self
    }

    fn collect_nodes(curr: Option<&Box<Node>>, more_than: i32, less_than: i32) -> Vec<i32> {
        curr.map(|node| {
            let mut res = Vec::<i32>::new();
            if more_than <= node.val && node.val <= less_than {
                res.push(node.val);
                res.extend(Tree::collect_nodes(node.left.as_ref(), more_than, less_than).iter());
                res.extend(Tree::collect_nodes(node.right.as_ref(), more_than, less_than).iter());
            } else if more_than < node.val {
                res.extend(Tree::collect_nodes(node.left.as_ref(), more_than, less_than).iter());
            } else if node.val < less_than {
                res.extend(Tree::collect_nodes(node.right.as_ref(), more_than, less_than).iter());
            }
            res
        })
        .unwrap_or(Vec::<i32>::new())
    }

    fn search(&self, more_than: i32, less_than: i32) -> Vec<i32> {
        let curr: Option<&Box<Node>> = self.root.as_ref();
        Tree::collect_nodes(curr, more_than, less_than)
    }
}

pub fn main() {
    let v = vec![9, 6, 14, 20, 1, 30, 8, 17, 5];
    let mut tree = Tree::new();
    for i in v {
        tree.insert(i);
    }
    println!("{:?}", tree.search(14, 20));
}
