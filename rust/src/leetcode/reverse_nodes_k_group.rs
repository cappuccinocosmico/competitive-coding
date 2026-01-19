// Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
//
// k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
//
// You may not alter the values in the list's nodes, only nodes themselves may be changed.
//
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

fn pop_and_advance_linked_node(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut owned_node = node.take()?;
    let next_node = owned_node.next.take();
    *node = next_node;
    Some(owned_node)
}

/// Pops off the first k elements if they exist, otherwise it will return none and leave the
/// original list unchanged.
fn pop_off_k_in_linked_list(
    begin_node: &mut Option<Box<ListNode>>,
    k: i32,
) -> Option<Box<ListNode>> {
    let mut owned_begin_node = begin_node.take();
    let mut seeking_ref = &mut owned_begin_node;
    let (seeked_forward, reached_end) = seek_pointer_forward_by_k(seeking_ref, k);
    seeking_ref = match reached_end {
        true => seeked_forward,
        false => {
            *begin_node = owned_begin_node;
            return None;
        }
    };
    let owned_tail = seeking_ref.take();
    *begin_node = owned_tail;
    owned_begin_node
}
fn seek_pointer_forward_by_k(
    mut pointer: &mut Option<Box<ListNode>>,
    k: i32,
) -> (&mut Option<Box<ListNode>>, bool) {
    for _ in 0..k {
        let Some(actual_val) = pointer else {
            return (pointer, false);
        };
        pointer = &mut actual_val.next
    }
    (pointer, true)
}

/// Takes in two linked lists, appends the first one onto the second, and returns an owned complete
/// list, and a reference to the cut point in the list. Modifies the root append node to be the
/// root of the new list with the appended items.
fn append_reverse_onto_linked_list(
    mut to_append: Option<Box<ListNode>>,
    root_append: &mut Option<Box<ListNode>>,
) {
    let Some(mut first_append) = pop_and_advance_linked_node(&mut to_append) else {
        return;
    };
    first_append.next = root_append.take();
    *root_append = Some(first_append);
    while let Some(mut next_append) = pop_and_advance_linked_node(&mut to_append) {
        next_append.next = root_append.take();
        *root_append = Some(next_append);
    }
}
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let head_to_consume_ref = &mut head;
        let mut return_list_head = None;
        let mut return_list_tail_ref = &mut return_list_head;
        loop {
            let owned_chunk = match pop_off_k_in_linked_list(head_to_consume_ref, k) {
                Some(val) => val,
                None => {
                    *return_list_tail_ref = head_to_consume_ref.take();
                    return return_list_head;
                }
            };
            let mut root_of_reversed = None;
            append_reverse_onto_linked_list(Some(owned_chunk), &mut root_of_reversed);
            *return_list_tail_ref = root_of_reversed;
            (return_list_tail_ref, _) = seek_pointer_forward_by_k(return_list_tail_ref, k + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a linked list from a vector
    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = ListNode::new(val);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }

    // Helper function to convert a linked list to a vector
    fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = list {
            vec.push(node.val);
            list = node.next;
        }
        vec
    }
    #[test]
    fn check_that_pop_returns_right_length() {
        let test_vec: Vec<_> = (1..100).collect();
        for i in 0..50 {
            let mut test_list = to_list(&test_vec);
            let popped_value = pop_off_k_in_linked_list(&mut test_list, i);
            let popped_to_vec = to_vec(popped_value);
            assert_eq!(popped_to_vec.len(), i as usize);
        }
    }

    #[test]
    fn test_reverse_k_group_k_2() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        let k = 2;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_reverse_k_group_k_3() {
        let head = to_list(&[1, 2, 3, 4, 5]);
        let k = 3;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn test_reverse_k_group_k_2_simple() {
        let head = to_list(&[1, 2]);
        let k = 2;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![2, 1]);
    }

    #[test]
    fn test_empty_list() {
        let head = to_list(&[]);
        let k = 1;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![]);
    }

    #[test]
    fn test_single_node_k_1() {
        let head = to_list(&[1]);
        let k = 1;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![1]);
    }

    #[test]
    fn test_single_node_k_2() {
        let head = to_list(&[1]);
        let k = 2;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![1]);
    }

    #[test]
    fn test_k_equals_length() {
        let head = to_list(&[1, 2, 3]);
        let k = 3;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![3, 2, 1]);
    }

    #[test]
    fn test_k_is_1() {
        let head = to_list(&[1, 2, 3]);
        let k = 1;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(to_vec(result), vec![1, 2, 3]);
    }
}
