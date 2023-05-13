use std::ptr::NonNull;

struct Node {
    letter: u8,
    is_word: bool,
    children: [Option<NonNull<Node>>; 256],
}

impl Node {
    pub fn new(letter: u8) -> NonNull<Self> {
        NonNull::new(Box::into_raw(Box::new(Self {
            letter,
            is_word: false,
            children: [None; 256],
        })))
        .unwrap()
    }
}

pub struct Trie {
    root: NonNull<Node>,
}

impl Trie {
    pub fn new() -> Self {
        let root = Node::new(0);

        Self { root }
    }

    pub fn insert(&mut self, word: &str) {
        assert!(word.is_ascii());
        let mut node = self.root;
        for &letter in word.as_bytes() {
            let p = unsafe { node.as_mut() };
            let index = letter as usize;
            if p.children[index].is_none() {
                p.children[index] = Some(Node::new(letter));
            }
            node = p.children[index].unwrap();
        }
        unsafe { node.as_mut() }.is_word = true;
    }

    pub fn exists(&self, word: &str) -> bool {
        assert!(word.is_ascii());
        let mut node = self.root;
        for &letter in word.as_bytes() {
            let p = unsafe { node.as_mut() };
            let index = letter as usize;
            if p.children[index].is_none() {
                return false;
            }
            node = p.children[index].unwrap();
        }
        unsafe { node.as_mut() }.is_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut trie = Trie::new();
        assert!(!trie.exists("abc"));
        assert!(!trie.exists("lodrem"));
        trie.insert("abc");
        trie.insert("lodrem");
        assert!(trie.exists("abc"));
        assert!(trie.exists("lodrem"));
        assert!(!trie.exists("ab"));
        assert!(!trie.exists("lodre"));
        assert!(!trie.exists("lodremm"));
    }
}
