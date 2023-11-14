use self::tree::{Node, Tree};

mod tree;

pub fn main() {
    let mut tree: Tree<u8> = Tree::new();
    tree.insert(5)
        .insert(3)
        .insert(6)
        .insert(7)
        .insert(1)
        .insert(2);
    tree.display();
    tree.display();
}
