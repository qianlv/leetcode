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

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lowest_common_ancestor_helper(root, &p, &q).0
    }

    fn lowest_common_ancestor_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> (Option<Rc<RefCell<TreeNode>>>, u8) {
        let mut n = 0;
        if &root == p || &root == q {
            n += 1;
        }
        if let Some(root) = root {
            let (left, ln) = Self::lowest_common_ancestor_helper(root.borrow().left.clone(), p, q);
            n += ln;
            let (right, rn) =
                Self::lowest_common_ancestor_helper(root.borrow().right.clone(), p, q);
            n += rn;
            if ln == 2 || (ln == 1 && n == 1) {
                return (left, ln);
            }
            if rn == 2 || (rn == 1 && n == 1) {
                return (right, rn);
            }
            if n == 2 || n == 1 {
                return (Some(root), n);
            }
        }
        (None, 0)
    }
}
