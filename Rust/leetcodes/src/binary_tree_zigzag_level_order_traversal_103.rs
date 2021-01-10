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

use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut rets: Vec<Vec<i32>> = Vec::new();
        if root.is_none() {
            return rets;
        }

        type TNode = Rc<RefCell<TreeNode>>;
        let mut que: [VecDeque<TNode>; 2] = [VecDeque::new(), VecDeque::new()];
        let mut dire = 0;
        que[dire].push_back(root.unwrap().clone());

        let mut ret: Vec<i32> = Vec::new();
        while !que[dire].is_empty() || !que[1 - dire].is_empty() {
            let node = que[dire].pop_back();

            if let Some(treenode) = node {
                let treenode = treenode.borrow();
                ret.push(treenode.val);
                if dire == 1 {
                    if let Some(ref right) = treenode.right {
                        que[1 - dire].push_back(right.clone());
                    }
                    if let Some(ref left) = treenode.left {
                        que[1 - dire].push_back(left.clone());
                    }
                } else {
                    if let Some(ref left) = treenode.left {
                        que[1 - dire].push_back(left.clone());
                    }
                    if let Some(ref right) = treenode.right {
                        que[1 - dire].push_back(right.clone());
                    }
                }
            }

            if que[dire].is_empty() {
                rets.push(ret.clone());
                ret.clear();
                dire = 1 - dire;
            }
        }
        if !ret.is_empty() {
            rets.push(ret);
        }
        rets
    }
}
