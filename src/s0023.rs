#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn priority_queue_merge_k_lists(
        mut lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        #[derive(Eq, PartialEq, Ord, PartialOrd)]
        struct Node {
            val: i32,
            idx: usize,
        }

        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();

        for (idx, node) in lists.iter().enumerate() {
            if let Some(n) = node {
                heap.push(Node {
                    val: -n.val,
                    idx: idx,
                })
            }
        }

        let mut head = None;
        let mut current = &mut head;

        while let Some(Node { val, idx }) = heap.pop() {
            current.replace(Box::new(ListNode::new(-val)));
            current = &mut current.as_mut().unwrap().next;

            let next = lists[idx].as_mut().unwrap().next.take();
            if let Some(n) = next.as_ref() {
                heap.push(Node {
                    val: -n.val,
                    idx: idx,
                });
            }
            lists[idx] = next;
        }

        head
    }

    #[allow(dead_code)]
    pub fn brute_force_merge_k_lists(
        mut lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        } else if lists.len() == 1 {
            return lists[0].take();
        }

        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        loop {
            let mut v = std::i32::MAX;
            let mut idx = 0;
            let mut is_empty = true;
            for i in 0..lists.len() {
                if let Some(node) = &lists[i] {
                    if node.val < v {
                        v = node.val;
                        idx = i;
                    }
                    is_empty = false;
                }
            }

            if is_empty {
                break;
            }

            let target_node = lists[idx].take().unwrap();

            tail.next = Some(Box::new(ListNode::new(target_node.val)));
            lists[idx] = target_node.next;
            tail = tail.next.as_mut().unwrap()
        }

        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn it_works() {
        let input = vec![
            Some(Box::new({
                let mut n1 = ListNode::new(1);
                n1.next = Some(Box::new({
                    let mut n2 = ListNode::new(4);
                    n2.next = Some(Box::new(ListNode::new(5)));

                    n2
                }));
                n1
            })),
            Some(Box::new({
                let mut n1 = ListNode::new(1);
                n1.next = Some(Box::new({
                    let mut n2 = ListNode::new(3);
                    n2.next = Some(Box::new(ListNode::new(4)));

                    n2
                }));
                n1
            })),
            Some(Box::new({
                let mut n1 = ListNode::new(2);
                n1.next = Some(Box::new(ListNode::new(6)));
                n1
            })),
        ];

        let mut output = Solution::brute_force_merge_k_lists(input);

        while let Some(_) = output.as_ref() {
            output = output.unwrap().next;
        }
    }
}
