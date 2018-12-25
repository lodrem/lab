#[derive(PartialEq, Eq, Debug)]
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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = {
                let val1 = match l1.as_ref() {
                    Some(l1) => l1.val,
                    None => 0,
                };

                let val2 = match l2.as_ref() {
                    Some(l2) => l2.val,
                    None => 0,
                };
                val1 + val2 + carry
            };

            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();

            if l1.is_some() {
                l1 = l1.unwrap().next;
            }

            if l2.is_some() {
                l2 = l2.unwrap().next;
            }
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
            let mut l1 = Box::new(ListNode::new(2));
            l1.next = Some(Box::new({
                let mut n = ListNode::new(4);
                n.next = Some(Box::new(ListNode::new(3)));
                n
            }));
            Some(l1)
        };
        let l2 = {
            let mut l2 = Box::new(ListNode::new(5));
            l2.next = Some(Box::new({
                let mut n = ListNode::new(6);
                n.next = Some(Box::new(ListNode::new(4)));
                n
            }));
            Some(l2)
        };
        let res = Solution::add_two_numbers(l1, l2);

        assert_eq!(res.as_ref().unwrap().val, 7);
        assert_eq!(res.as_ref().unwrap().next.as_ref().unwrap().val, 0);
        assert_eq!(
            res.as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val,
            8
        );
    }
}
