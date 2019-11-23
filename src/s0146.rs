use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct LRUEntry {
    prev: Option<Rc<RefCell<LRUEntry>>>,
    next: Option<Rc<RefCell<LRUEntry>>>,
    key: i32,
    value: i32,
}

impl LRUEntry {
    fn new(key: i32, value: i32) -> Self {
        Self {
            prev: None,
            next: None,
            key,
            value,
        }
    }
}

pub struct LRUCache {
    head: Rc<RefCell<LRUEntry>>,
    tail: Rc<RefCell<LRUEntry>>,
    storage: HashMap<i32, Rc<RefCell<LRUEntry>>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LRUCache {
    fn push(&mut self, node: Rc<RefCell<LRUEntry>>) {
        node.borrow_mut().prev = Some(self.head.clone());
        node.borrow_mut().next = self.head.borrow().next.clone();

        self.head.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        self.head.borrow_mut().next = Some(node);
    }

    fn detach(&mut self, node: Rc<RefCell<LRUEntry>>) {
        let prev_node = node.borrow().prev.as_ref().unwrap().clone();
        let next_node = node.borrow().next.as_ref().unwrap().clone();

        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(prev_node);
    }

    fn pop(&mut self) -> Rc<RefCell<LRUEntry>> {
        let tail_node = self.tail.borrow().prev.clone().unwrap();

        self.detach(tail_node.clone());
        tail_node
    }

    pub fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(LRUEntry::new(0, 0)));
        let tail = Rc::new(RefCell::new(LRUEntry::new(0, 0)));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        Self {
            head,
            tail,
            storage: HashMap::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.storage.get(&key).cloned() {
            Some(node) => {
                let value = node.borrow().value;

                // Update node lru
                self.detach(node.clone());
                self.push(node);

                value
            }
            _ => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        match self.storage.get(&key).cloned() {
            Some(node) => {
                node.borrow_mut().value = value;

                // Update node lru
                self.detach(node.clone());
                self.push(node);
            }
            _ => {
                let node = Rc::new(RefCell::new(LRUEntry::new(key, value)));

                self.storage.insert(key, node.clone());
                self.push(node);

                if self.storage.len() > self.capacity {
                    let tail = self.pop();
                    self.storage.remove(&tail.borrow().key);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;
    #[test]
    fn it_works() {
        {
            let mut cache = LRUCache::new(2);

            cache.put(1, 1);
            cache.put(2, 2);
            cache.put(3, 3);
            assert_eq!(-1, cache.get(-1));
            assert_eq!(2, cache.get(2));
            assert_eq!(3, cache.get(3));
            cache.put(4, 4);
            assert_eq!(4, cache.get(4));
            assert_eq!(-1, cache.get(2));
            assert_eq!(3, cache.get(3));
            cache.put(5, 5);
            assert_eq!(-1, cache.get(4));
            assert_eq!(3, cache.get(3));
            assert_eq!(5, cache.get(5));
        }
    }
}
