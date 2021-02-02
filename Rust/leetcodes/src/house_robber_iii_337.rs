#![allow(dead_code)]

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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::rob_helper(&root).1;
    }

    fn rob_helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root.as_ref() {
            let (left_pprev, left_prev) = Self::rob_helper(&root.borrow().left);
            let (right_pprev, right_prev) = Self::rob_helper(&root.borrow().right);
            return (
                left_prev + right_prev,
                std::cmp::max(
                    root.borrow().val + left_pprev + right_pprev,
                    left_prev + right_prev,
                ),
            );
        }
        (0, 0)
    }
}
