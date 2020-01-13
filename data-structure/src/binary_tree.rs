struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    pub fn append_left(&mut self, child: Box<Node<T>>) {
        self.left = Some(child);
    }

    pub fn append_right(&mut self, child: Box<Node<T>>) {
        self.right = Some(child);
    }
}

struct BinaryTree<T> {
    root: Box<Node<T>>,
}

impl<T> BinaryTree<T> {
    pub fn new(root: Box<Node<T>>) -> Self {
        BinaryTree { root }
    }
}
