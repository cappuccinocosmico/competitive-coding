use std::collections::BTreeMap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListIndex {
    pub val: i32,
    pub address: usize,
}

impl PartialOrd for ListIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val
            .cmp(&other.val)
            .then_with(|| self.address.cmp(&other.address))
    }
}

fn new_list_index(node: &ListNode) -> ListIndex {
    return ListIndex {
        val: node.val,
        address: (node as *const ListNode) as usize,
    };
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut list_btree: BTreeMap<ListIndex, Box<ListNode>> = lists
        .into_iter()
        .flatten()
        .map(|val| (new_list_index(&val), val))
        .collect();
    let mut return_head: Option<Box<ListNode>> = None;
    let mut option_tail = &mut return_head;
    while let Some((_, mut leftover_node)) = list_btree.pop_first() {
        let taken_tail = leftover_node.next.take();
        if let Some(tail) = option_tail {
            tail.next = Some(leftover_node);
            option_tail = &mut tail.next
        } else {
            *option_tail = Some(leftover_node);
        };
        if let Some(leftover_tail) = taken_tail {
            list_btree.insert(new_list_index(&leftover_tail), leftover_tail);
        };
    }

    return_head
}
