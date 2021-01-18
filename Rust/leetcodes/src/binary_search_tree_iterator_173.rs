// Definition for a binary tree node.
use std::cell::{Ref, RefCell};
use std::rc::Rc;

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

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    // stack: Option<Vec<Rc<RefCell<TreeNode>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// impl BSTIterator {
//     fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
//         let mut iter = BSTIterator { stack: Vec::new() };
//         if let Some(ref root) = root {
//             iter.stack.push(Some(root.clone()));
//             iter.push_left(&root);
//         }
//         iter
//     }

//     fn next(&mut self) -> i32 {
//         let root = self.stack.pop().expect("BSTIterator non-empty").unwrap();
//         let ret = root.borrow().val;
//         if let Some(ref right) = root.borrow().right {
//             self.stack.push(Some(right.clone()));
//             self.push_left(&right);
//         }
//         ret
//     }

//     fn push_left(&mut self, root: &Rc<RefCell<TreeNode>>) {
//         while let Some(ref left) = root.borrow().left {
//             self.stack.push(Some(left.clone()));
//             self.push_left(left);
//         }
//     }

//     fn has_next(&self) -> bool {
//         !self.stack.is_empty()
//     }
// }

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = BSTIterator { stack: Vec::new() };
        iter.push_left(root);
        iter
    }

    fn next(&mut self) -> i32 {
        let root = self.stack.pop().expect("BSTIterator non-empty");
        let ret = root.borrow().val;
        let root = root.borrow();
        if let Some(right) = root.right.as_ref() {
            self.push_left(Some(right.clone()));
        }
        ret
    }

    fn push_left(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(left) = root {
            self.stack.push(left.clone());
            root = left.borrow().left.as_ref().map(|l| Rc::clone(l));
        }
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

//
// Your BSTIterator object will be instantiated and called as such:
// let obj = BSTIterator::new(root);
// let ret_1: i32 = obj.next();
// let ret_2: bool = obj.has_next();
//
