#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    use std::mem;

    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    let (l1, l2) = (&mut l1, &mut l2);
    while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
      if n1.val < n2.val {
        mem::swap(&mut tail.next, l1);
        mem::swap(&mut tail.next.as_mut().unwrap().next, l1);
      } else {
        mem::swap(&mut tail.next, l2);
        mem::swap(&mut tail.next.as_mut().unwrap().next, l2);
      }

      tail = tail.next.as_mut().unwrap();
    }

    if l1.is_some() {
      mem::swap(&mut tail.next, l1);
    } else if l2.is_some() {
      mem::swap(&mut tail.next, l2);
    }

    head.next
  }
}

#[cfg(test)]
mod tests {
  use super::{ListNode, Solution};

  #[test]
  fn it_works() {
    let l1 = {
      let mut head = Box::new(ListNode::new(1));
      let mut node2 = Box::new(ListNode::new(2));
      let node3 = Box::new(ListNode::new(4));
      node2.next = Some(node3);
      head.next = Some(node2);
      Some(head)
    };

    let l2 = {
      let mut head = Box::new(ListNode::new(1));
      let mut node2 = Box::new(ListNode::new(3));
      let node3 = Box::new(ListNode::new(4));
      node2.next = Some(node3);
      head.next = Some(node2);
      Some(head)
    };

    let result = {
      let mut head = Box::new(ListNode::new(1));
      let mut node2 = Box::new(ListNode::new(1));
      let mut node3 = Box::new(ListNode::new(2));
      let mut node4 = Box::new(ListNode::new(3));
      let mut node5 = Box::new(ListNode::new(4));
      let node6 = Box::new(ListNode::new(4));
      node5.next = Some(node6);
      node4.next = Some(node5);
      node3.next = Some(node4);
      node2.next = Some(node3);
      head.next = Some(node2);
      Some(head)
    };

    assert_eq!(result, Solution::merge_two_lists(l1, l2));
  }
}
