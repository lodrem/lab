// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

        if root.is_none() {
            return 0;
        }

        let mut max_depth = 1;

        let mut nodes = VecDeque::default();

        nodes.push_back((root.unwrap(), 1));

        while !nodes.is_empty() {
            let (node, depth) = nodes.pop_front().unwrap();

            if depth > max_depth {
                max_depth = depth;
            }
            let node = node.borrow();
            if let Some(left) = node.left.clone() {
                nodes.push_back((left, depth + 1))
            }
            if let Some(right) = node.right.clone() {
                nodes.push_back((right, depth + 1))
            }
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::{Solution, TreeNode};

    #[test]
    fn it_works() {
        {
            let root = {
                let root = Rc::new(RefCell::new(TreeNode::new(3)));

                root.borrow_mut().left = {
                    let node = Rc::new(RefCell::new(TreeNode::new(9)));
                    Some(node)
                };

                root.borrow_mut().right = {
                    let node = Rc::new(RefCell::new(TreeNode::new(20)));

                    node.borrow_mut().left = {
                        let node = Rc::new(RefCell::new(TreeNode::new(15)));
                        Some(node)
                    };
                    node.borrow_mut().right = {
                        let node = Rc::new(RefCell::new(TreeNode::new(7)));
                        Some(node)
                    };
                    Some(node)
                };

                root
            };

            assert_eq!(3, Solution::max_depth(Some(root)));
        }
    }
}
