use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let nums = Self::sorted_list_to_array(head);
        if nums.is_empty() {
            return None;
        }
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

    fn sorted_list_to_array(head: Option<Box<ListNode>>) -> Vec<i32> {
        if head.is_none() {
            return vec![];
        }

        let mut res = vec![];

        let mut next = head.unwrap();

        loop {
            res.push(next.val);

            if next.next.is_none() {
                break;
            }

            next = next.next.unwrap();
        }

        res
    }
}
