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
        self.append_child(n);
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
}
