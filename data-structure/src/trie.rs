const ASCII_A: usize = 97;

fn get_digit(c: char) -> Option<usize> {
    match c as usize {
        97..=122 => Some((c as usize) - ASCII_A),
        _ => None,
    }
}

fn is_lower_alphabet(s: &str) -> bool {
    s.chars().all(|c| get_digit(c).is_some())
}

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

    fn append_child(&mut self, i: usize) {
        assert!(i <= 25);
        self.children[i] = Some(Box::new(Node::new()));
    }

    fn insert(&mut self, s: &str) {
        let c = s.chars().nth(0);
        if c.is_none() {
            self.is_end_of_word = true;
            return;
        }

        let s = &s[1..];
        let n = get_digit(c.unwrap()).unwrap();
        if self.children[n].is_none() {
            self.append_child(n);
        }
        self.children[n].as_mut().unwrap().insert(s);
    }

    fn search(&self, s: &str) -> bool {
        let c = s.chars().nth(0);
        if c.is_none() {
            return self.is_end_of_word;
        }

        let s = &s[1..];
        let n = get_digit(c.unwrap()).unwrap();
        let child = &self.children[n];
        if child.is_none() {
            return false;
        }

        child.as_ref().unwrap().search(s)
    }

    /// returns true if there are at least one child node
    /// in children array
    fn is_leaf(&self) -> bool {
        self.children.iter().all(|c| c.is_none())
    }

    fn get_node(&self, s: &str) -> Option<&Node> {
        let c = s.chars().nth(0);
        if c.is_none() {
            return Some(self);
        }

        let s = &s[1..];
        let n = get_digit(c.unwrap()).unwrap();
        let child = &self.children[n];
        if child.is_none() {
            return None;
        }

        child.as_ref().unwrap().get_node(s)
    }

    fn delete(&mut self, s: &str) -> bool {
        if self.is_leaf() {
            return false;
        }
        // search given string and call delete
        // if child.delete() {
        //   delete child position
        //   if other children exists, return false
        //   if self is leaf, return true
        // } else { do nothing }

        false
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn insert(&mut self, s: &str) {
        assert!(is_lower_alphabet(s));
        self.root.insert(s);
    }

    pub fn search(&self, s: &str) -> bool {
        assert!(is_lower_alphabet(s));
        self.root.search(s)
    }

    pub fn delete(&mut self, s: &str) {
        self.root.delete(s);
    }

    pub fn get_node(&self, s: &str) -> Option<&Node> {
        self.root.get_node(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit() {
        assert_eq!(get_digit('a'), Some(0));
        assert_eq!(get_digit('z'), Some(25));
        assert_eq!(get_digit('-'), None);
    }

    #[test]
    fn test_is_lower_alphabet() {
        assert!(is_lower_alphabet("helloworld"));
        assert!(!is_lower_alphabet("HelloWorld"));
    }

    #[test]
    fn test_is_leaf() {
        let mut trie = Trie::new();
        assert!(trie.root.is_leaf());

        trie.insert("a");
        assert!(!trie.root.is_leaf());
    }

    #[test]
    fn test_get_node() {
        let mut trie = Trie::new();
        assert!(trie.root.is_leaf());

        trie.insert("hello");
        assert!(trie.get_node("hel").is_some());
        assert!(trie.get_node("hello").is_some());
        assert!(trie.get_node("abc").is_none());
    }

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();

        trie.insert("hello");
        trie.insert("world");
        assert!(trie.search("hello"));
        assert!(!trie.search("hi"));
    }

    #[test]
    fn test_trie2() {
        let mut trie = Trie::new();
        trie.insert("there");
        trie.insert("their");

        assert!(trie.search("their"));
        assert!(!trie.search("thei"));
    }

    #[test]
    fn test_add_existing_node() {
        let mut trie = Trie::new();
        trie.insert("there");
        trie.insert("the");

        assert!(trie.search("there"));
    }

    #[test]
    fn test_delete() {
        let mut trie = Trie::new();
        trie.insert("the");

        trie.delete("the");
        assert!(!trie.search("the"));
        assert!(trie.root.is_leaf());
    }

    #[test]
    fn test_delete_mid() {
        let mut trie = Trie::new();
        trie.insert("there");
        trie.insert("the");
        trie.delete("the");

        assert!(!trie.search("the"));
        assert!(trie.search("there"));
    }

    #[test]
    fn test_delete_leaf() {
        let mut trie = Trie::new();
        trie.insert("there");
        trie.insert("the");
        trie.delete("there");

        assert!(!trie.search("there"));
        assert!(trie.search("the"));
        // TODO: assert 'the' node is now leaf
    }
}
