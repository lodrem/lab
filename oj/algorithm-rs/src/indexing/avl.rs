use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T>
where
    T: Ord,
{
    value: T,
    height: usize,
    parent: Option<Rc<RefCell<Node<T>>>>,
    left_child: Option<Rc<RefCell<Node<T>>>>,
    right_child: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    pub fn new(value: T, parent: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self {
            value,
            parent,
            height: 0,
            left_child: None,
            right_child: None,
        }
    }

    pub fn insert(&mut self, value: T) {}
}

#[derive(Debug)]
pub struct AVLTree<T>
where
    T: Ord,
{
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> AVLTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {}
}
