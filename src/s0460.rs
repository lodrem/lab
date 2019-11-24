use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct LFUEntry {
    key: i32,
    value: i32,
    counter: usize,
    prev: Option<Rc<RefCell<LFUEntry>>>,
    next: Option<Rc<RefCell<LFUEntry>>>,
}

impl LFUEntry {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            counter: 0,
            prev: None,
            next: None,
        }
    }

    fn detach(&mut self) {
        if self.prev.is_none() || self.next.is_none() {
            return;
        }
        let prev = self.prev.as_ref().unwrap();
        let next = self.next.as_ref().unwrap();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());
    }
}

struct LFUFrequency {
    head: Rc<RefCell<LFUEntry>>,
    tail: Rc<RefCell<LFUEntry>>,
}

impl LFUFrequency {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(LFUEntry::new(0, 0)));
        let tail = Rc::new(RefCell::new(LFUEntry::new(0, 0)));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        Self { head, tail }
    }

    fn push(&mut self, entry: Rc<RefCell<LFUEntry>>) {
        {
            let mut entry = entry.borrow_mut();

            entry.prev = Some(self.head.clone());
            entry.next = self.head.borrow().next.clone();
        }

        self.head.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(entry.clone());
        self.head.borrow_mut().next = Some(entry);
    }

    fn pop(&mut self) -> Rc<RefCell<LFUEntry>> {
        let entry = self.tail.borrow().prev.as_ref().unwrap().clone();
        entry.borrow_mut().detach();
        entry
    }

    fn is_empty(&self) -> bool {
        Rc::ptr_eq(&self.head.borrow().next.as_ref().unwrap(), &self.tail)
    }
}

struct LFUCache {
    storage: HashMap<i32, Rc<RefCell<LFUEntry>>>,
    entry_frequency: HashMap<usize, LFUFrequency>,
    capacity: usize,
    min_frequency: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            storage: HashMap::with_capacity(capacity as usize),
            entry_frequency: Default::default(),
            capacity: capacity as usize,
            min_frequency: 1,
        }
    }

    fn update_frequency(&mut self, entry: Rc<RefCell<LFUEntry>>) {
        {
            let mut entry = entry.borrow_mut();
            entry.detach();

            if let Some(frequency) = self.entry_frequency.get(&entry.counter) {
                if frequency.is_empty() {
                    self.entry_frequency.remove(&entry.counter);

                    if self.min_frequency == entry.counter {
                        self.min_frequency += 1;
                    }
                }
            }

            entry.counter += 1;
        }

        let counter = entry.borrow().counter;
        match self.entry_frequency.get_mut(&counter) {
            Some(frequency) => {
                frequency.push(entry);
            }
            None => {
                let mut frequency = LFUFrequency::new();

                frequency.push(entry);

                self.entry_frequency.insert(counter, frequency);
            }
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.storage.get(&key).cloned() {
            Some(entry) => {
                self.update_frequency(entry.clone());

                entry.borrow().value
            }
            _ => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return
        }
        match self.storage.get(&key).cloned() {
            Some(entry) => {
                entry.borrow_mut().value = value;
                self.update_frequency(entry);
            }
            None => {
                if self.storage.len() + 1 > self.capacity {
                    let frequency = self.entry_frequency.get_mut(&self.min_frequency).unwrap();

                    let entry = frequency.pop();
                    self.storage.remove(&entry.borrow().key);

                    if frequency.is_empty() {
                        self.min_frequency = 1;
                    }
                }

                let entry = Rc::new(RefCell::new(LFUEntry::new(key, value)));

                self.storage.insert(key, entry.clone());
                self.update_frequency(entry.clone());

                self.min_frequency = 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LFUCache;
    #[test]
    fn it_works() {
        {
            let mut cache = LFUCache::new(3);

            cache.put(2, 2);
            cache.put(1, 1);
            assert_eq!(2, cache.get(2)); // 2 -> 2
            assert_eq!(1, cache.get(1)); // 1 -> 2
            assert_eq!(2, cache.get(2)); // 2 -> 3
            cache.put(3, 3); // 3 -> 1
            cache.put(4, 4); // 4 -> 1
            assert_eq!(-1, cache.get(3));
            assert_eq!(2, cache.get(2));
            assert_eq!(1, cache.get(1));
            assert_eq!(4, cache.get(4));
        }

        {
            // ["LFUCache","put","put","put","put","get","get"]
            // [[2],[2,1],[1,1],[2,3],[4,1],[1],[2]]

            let mut cache = LFUCache::new(2);

            cache.put(2, 1);
            cache.put(1, 1);
            cache.put(2, 3);
            cache.put(4, 1);
            assert_eq!(-1, cache.get(1));
            assert_eq!(3, cache.get(2));
        }
    }
}
