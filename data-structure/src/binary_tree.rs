struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn append_left(&mut self, child: Box<Node>) {
        self.left = Some(child);
    }

    fn append_right(&mut self, child: Box<Node>) {
        self.right = Some(child);
    }
}

struct BinaryTree {
    root: Box<Node>,
}

impl BinaryTree {
    pub fn new(root: Box<Node>) -> Self {
        BinaryTree { root }
    }
}
