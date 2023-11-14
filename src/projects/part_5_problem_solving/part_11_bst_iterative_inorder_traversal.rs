use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

pub struct Tree<T: Debug + PartialOrd + Clone> {
    root: Option<Rc<RefCell<Node<T>>>>,
    stack: Vec<Rc<RefCell<Node<T>>>>,
}

pub struct Node<T: Debug + PartialOrd + Clone> {
    pub val: T,
    pub parent: Option<Weak<RefCell<Node<T>>>>, // weak parent reference
    // strong children node reference
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug + PartialOrd + Clone> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.val)
    }
}

impl<T: Debug + PartialOrd + Clone> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None,
            stack: Vec::new(),
        }
    }

    pub fn with(words: Vec<T>) -> Tree<T> {
        let mut tree = Tree::<T>::new();
        for word in words {
            tree.insert(word);
        }
        let mut curr = tree.root.as_ref().map(|node| node.clone());
        while curr.is_some() {
            curr = curr
                .as_ref()
                .map(|node| {
                    tree.stack.push(node.clone());
                    (**node)
                        .borrow()
                        .left
                        .as_ref()
                        .map_or(None, |node| Some(node.clone()))
                })
                .unwrap();
        }
        tree
    }

    pub fn insert(&mut self, val: T) -> &mut Self {
        let mut curr: Option<Rc<RefCell<Node<T>>>> = self.root.clone();
        if curr.is_none() {
            self.root = Some(Rc::new(RefCell::new(Node {
                val,
                parent: None,
                left: None,
                right: None,
            })));
        } else {
            while let Some(curr_node) = curr {
                if val < curr_node.borrow().val {
                    if curr_node.borrow().left.is_some() {
                        curr = curr_node.borrow().left.clone();
                    } else {
                        (*curr_node).borrow_mut().left = Some(Rc::new(RefCell::new(Node {
                            val,
                            parent: Some(Rc::downgrade(&curr_node)),
                            left: None,
                            right: None,
                        })));
                        break;
                    }
                } else {
                    if curr_node.borrow().right.is_some() {
                        curr = curr_node.borrow().right.clone();
                    } else {
                        (*curr_node).borrow_mut().right = Some(Rc::new(RefCell::new(Node {
                            val,
                            parent: Some(Rc::downgrade(&curr_node)),
                            left: None,
                            right: None,
                        })));
                        break;
                    }
                }
            }
        }
        self
    }

    fn display_nodes(curr: Option<&Rc<RefCell<Node<T>>>>, node: &str) {
        curr.map(|curr_node| {
            Tree::display_nodes((**curr_node).borrow().left.as_ref(), "left");
            print!(
                "current_node_val: {:?}, node: {:?}",
                (**curr_node).borrow().val,
                node
            );
            (**curr_node).borrow().parent.as_ref().map(|x| {
                x.upgrade().map(|y| {
                    print!(" parent_node_val: {:?}", (*y).borrow().val);
                })
            });
            println!();
            Tree::display_nodes((**curr_node).borrow().right.as_ref(), "right");
        });
    }

    pub fn display(&mut self) {
        let curr: Option<&Rc<RefCell<Node<T>>>> = self.root.as_ref().clone();
        Tree::display_nodes(curr, "tree");
        println!();
    }

    pub fn next(&mut self, n: usize) -> Vec<T> {
        let mut res = Vec::<T>::new();
        while !self.stack.is_empty() {
            if res.len() >= n {
                break;
            }
            let mut curr = self.stack.pop();
            curr = curr
                .as_ref()
                .map(|node| {
                    res.push((**node).borrow().val.clone());
                    (**node)
                        .borrow()
                        .right
                        .as_ref()
                        .map_or(None, |node| Some(node.clone()))
                })
                .unwrap();
            while curr.is_some() {
                curr = curr
                    .as_ref()
                    .map(|node| {
                        self.stack.push(node.clone());
                        (**node)
                            .borrow()
                            .left
                            .as_ref()
                            .map_or(None, |node| Some(node.clone()))
                    })
                    .unwrap();
            }
        }
        res
    }
}
pub fn main() {
    let mut tree = Tree::with(vec![
        "Latasha".to_string(),
        "Elia".to_string(),
        "Caryl".to_string(),
        "Elvira".to_string(),
        "Lala".to_string(),
        "Lyn".to_string(),
        "Jeanette".to_string(),
    ]);

    println!("{:?}", tree.next(2));
    println!("{:?}", tree.next(2));
    println!("{:?}", tree.next(2));
    println!("{:?}", tree.next(2));
    println!("{:?}", tree.next(2));    
    tree.display();
}
