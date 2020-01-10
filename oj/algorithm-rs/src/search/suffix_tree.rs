use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Debug)]
struct Node {
    children: HashMap<u8, Rc<RefCell<Node>>>,
    indexes: Vec<usize>,
}

impl Node {
    fn root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Default::default()))
    }

    fn insert(&mut self, s: &str, index: usize) {
        self.indexes.push(index);

        if s.is_empty() {
            return;
        }

        let bytes = s.as_bytes();

        let first_char = bytes[0];
        let child = self.children.entry(first_char).or_insert(Self::root());
        child.borrow_mut().insert(&s[1..], index + 1);
    }

    fn search(&self, s: &str) -> Vec<usize> {
        if s.is_empty() {
            self.indexes.clone()
        } else {
            let first_char = s.as_bytes()[0];

            if let Some(child) = self.children.get(&first_char) {
                child.borrow().search(&s[1..])
            } else {
                vec![]
            }
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: Rc<RefCell<Node>>,
}

impl Trie {
    fn new(s: &str) -> Self {
        let root = Node::root();
        {
            let mut root = root.borrow_mut();
            for i in 0..s.len() {
                root.insert(&s[i..], i);
            }
        }

        Self { root }
    }

    fn search(&self, pattern: &str) -> Vec<usize> {
        self.root
            .borrow()
            .search(pattern)
            .into_iter()
            .map(|index| index - pattern.len())
            .collect()
    }
}

#[allow(dead_code)]
pub fn search(s: &str, pattern: &str) -> Vec<usize> {
    let trie = Trie::new(s);

    trie.search(pattern)
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn it_works() {
        assert_eq!(search("abcdefg", "acde"), vec![]);
        assert_eq!(search("aaaaaaa", "a"), vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(search("abcdefg", "cde"), vec![2]);
        assert_eq!(search("THIS IS A TEST TEXT", "TEST"), vec![10]);
        assert_eq!(search("AABAACAADAABAABA", "AABA"), vec![0, 9, 12]);
        assert_eq!(search("abbaabbaaba", "abbaaba"), vec![4]);
        assert_eq!(search("ababababca", "abababca"), vec![2]);
    }
}
