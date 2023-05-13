use std::ptr::NonNull;

struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    front: Option<NonNull<Node<T>>>,
    back: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, v: T) {
        let node = NonNull::new(Box::into_raw(Box::new(Node {
            value: v,
            prev: None,
            next: self.front,
        })))
        .unwrap();

        if let Some(front) = self.front {
            unsafe { (*front.as_ptr()).prev = Some(node) };
        } else {
            self.back = Some(node);
        }
        self.front = Some(node);
        self.len += 1;
    }

    pub fn push_back(&mut self, v: T) {
        let node = NonNull::new(Box::into_raw(Box::new(Node {
            value: v,
            prev: self.back,
            next: None,
        })))
        .unwrap();

        if let Some(back) = self.back {
            unsafe { (*back.as_ptr()).next = Some(node) };
        } else {
            self.front = Some(node);
        }
        self.back = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = if let Some(front) = self.front {
            unsafe { Box::from_raw(front.as_ptr()) }
        } else {
            return None;
        };
        self.front = node.next;
        if let Some(front) = self.front {
            unsafe { (*front.as_ptr()).prev = None };
        } else {
            self.back = None;
        }
        self.len -= 1;
        Some(node.value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let node = if let Some(back) = self.back {
            unsafe { Box::from_raw(back.as_ptr()) }
        } else {
            return None;
        };

        self.back = node.prev;
        if let Some(back) = self.back {
            unsafe { (*back.as_ptr()).next = None };
        } else {
            self.front = None;
        }
        self.len -= 1;
        Some(node.value)
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &node.as_ref().value) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|mut node| &mut node.as_mut().value) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &node.as_ref().value) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|mut node| &mut node.as_mut().value) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_back_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.len(), 1);

        list.push_back(2);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&2));
        assert_eq!(list.len(), 2);

        list.push_back(3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
        assert_eq!(list.len(), 3);

        list.push_front(0);
        assert_eq!(list.front(), Some(&0));
        assert_eq!(list.back(), Some(&3));
        assert_eq!(list.len(), 4);

        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&2));
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.len(), 1);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_back_front_mut() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.front_mut(), Some(&mut 1));
        list.front_mut().map(|v| *v = 0);
        assert_eq!(list.front(), Some(&0));
        assert_eq!(list.back_mut(), Some(&mut 3));
        list.back_mut().map(|v| *v = 4);
        assert_eq!(list.back(), Some(&4));
    }
}
