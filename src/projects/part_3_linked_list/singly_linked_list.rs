pub type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T>
where
    T: Copy + std::fmt::Debug,
{
    pub val: T,
    pub next: Pointer<T>,
}

#[derive(Debug)]
pub struct LinkedList<T>
where
    T: Copy + std::fmt::Debug,
{
    pub head: Pointer<T>,
}

impl<T> LinkedList<T>
where
    T: Copy + std::fmt::Debug,
{
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn add(&mut self, val: T) -> &mut Self {
        let prev_head = self.head.take();
        self.head = Some(Box::new(Node {
            val,
            next: prev_head,
        }));
        self
    }

    pub fn remove(&mut self) -> Option<T> {
        let prev_head = self.head.take();
        match prev_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.val)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<T> {
        match &self.head {
            Some(x) => Some(x.val),
            None => None,
        }
    }

    pub fn display(&self) {
        let mut curr = &self.head;
        loop {
            match curr {
                Some(node) => {
                    print!("-> {:?} ", node.val);
                    curr = &node.next;
                }
                None => {
                    println!();
                    break;
                }
            }
        }
    }

}
