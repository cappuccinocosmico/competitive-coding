// Definition for singly-linked list.
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
struct Solution {}
//You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
const BASE: i32 = 10;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut borrow = 0;
        let mut return_node = None;
        let mut tail = &mut return_node;
        let (mut l1_ref, mut l2_ref) = (l1.as_ref(), l2.as_ref());
        while l1_ref.is_some() || l2_ref.is_some() || borrow != 0 {
            let l1_val = l1_ref.map_or(0, |val| val.val);
            let l2_val = l2_ref.map_or(0, |val| val.val);
            let value = borrow + l1_val + l2_val;
            let (rem, divisor) = (value % BASE, value / BASE);
            let new_node = ListNode::new(rem);
            *tail = Some(Box::new(new_node));
            tail = &mut tail.as_mut().unwrap().next;
            borrow = divisor;

            l1_ref = l1_ref.map_or(None, |val| (val.next).as_ref());
            l2_ref = l2_ref.map_or(None, |val| (val.next).as_ref());
        }

        return_node
    }
}
