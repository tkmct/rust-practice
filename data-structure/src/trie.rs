struct Node {
    children: [Option<Box<Node>>; 26],
    is_end_of_word: bool,
}

impl Node {
    fn new() -> Self {
        let children = Default::default();

        Self {
            children,
            is_end_of_word: false,
        }
    }
}

struct Tree {
    root: Node,
}

impl Tree {
    fn new() -> Self {
        Self { root: Node::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
