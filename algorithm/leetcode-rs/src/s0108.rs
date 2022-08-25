use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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

#[allow(dead_code)]
impl Solution {
    pub fn sorted_array_to_bst(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        nums.sort();

        Some(Self::partition(&nums))
    }

    fn partition(nums: &[i32]) -> Rc<RefCell<TreeNode>> {
        let root = Rc::new(RefCell::new(TreeNode::new(nums[nums.len() / 2])));

        if nums.len() / 2 > 0 {
            root.borrow_mut().left = Some(Self::partition(&nums[..nums.len() / 2]));
        }

        if nums.len() / 2 + 1 < nums.len() {
            root.borrow_mut().right = Some(Self::partition(&nums[nums.len() / 2 + 1..]));
        }

        root
    }
}
