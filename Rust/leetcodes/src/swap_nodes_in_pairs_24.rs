#![allow(dead_code)]

struct Solution;

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

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Solution::swap_pairs_helper(head);
    }

    pub fn swap_pairs_helper(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if let Some(mut first) = head.take() {
            if let Some(mut second) = first.next.take() {
                if let Some(next) = second.next.take() {
                    first.next = Solution::swap_pairs_helper(Some(next));
                } else {
                    first.next = None;
                }
                second.next = Some(first);
                return Some(second);
            }
            return Some(first);
        }
        return None;
    }
}
