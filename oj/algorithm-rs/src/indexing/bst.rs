use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T>
where
    T: Ord,
{
    value: T,
    left_child: Option<Rc<RefCell<Node<T>>>>,
    right_child: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        Self {
            value,
            left_child: None,
            right_child: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(node) = &self.left_child {
                    node.borrow_mut().insert(value);
                } else {
                    self.left_child = Some(Rc::new(RefCell::new(Self::new(value))))
                }
            }
            _ => {
                if let Some(node) = &self.right_child {
                    node.borrow_mut().insert(value);
                } else {
                    self.right_child = Some(Rc::new(RefCell::new(Self::new(value))))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match &self.root {
            Some(node) => node.borrow_mut().insert(value),
            _ => self.root = Some(Rc::new(RefCell::new(Node::new(value)))),
        }
    }
}
