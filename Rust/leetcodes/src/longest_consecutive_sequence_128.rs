#![allow(dead_code)]

struct Solution;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

trait Disjoint {
    fn init(&mut self, vals: &Vec<i32>);
    fn find(&mut self, x: i32) -> Option<(i32, u32)>;
    fn union(&mut self, x: i32, y: i32) -> u32;
}

type DisjointSet = HashMap<i32, RefCell<(i32, u32)>>;
impl Disjoint for DisjointSet {
    fn init(&mut self, vals: &Vec<i32>) {
        for &v in vals {
            self.insert(v, RefCell::new((v, 1)));
        }
    }
    fn find(&mut self, x: i32) -> Option<(i32, u32)> {
        if !self.contains_key(&x) {
            return None;
        }
        let (xp, cnt) = self.get(&x).unwrap().borrow().clone();
        if x == xp {
            return Some((xp, cnt));
        }
        let p = self.find(xp).unwrap();
        let mut val = self.get(&x).unwrap().borrow_mut();
        val.0 = p.0;
        return Some(p.clone());
    }

    fn union(&mut self, x: i32, y: i32) -> u32 {
        let xp = self.find(x);
        let yp = self.find(y);
        if xp.is_none() || yp.is_none() {
            return 0;
        }

        let xp = xp.unwrap();
        let yp = yp.unwrap();
        if xp.0 == yp.0 {
            return xp.1;
        }

        if let Some(v) = self.get_mut(&xp.0) {
            v.borrow_mut().0 = yp.0;
        }
        if let Some(v) = self.get_mut(&yp.0) {
            v.borrow_mut().1 += xp.1;
        }
        return xp.1 + yp.1;
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        Solution::longest_consecutive_with_set(nums)
        // let mut disjoint = DisjointSet::new();
        // disjoint.init(&nums);
        // let mut longest = {
        //     if nums.is_empty() {
        //         0
        //     } else {
        //         1
        //     }
        // };
        // for num in nums {
        //     longest = std::cmp::max(longest, disjoint.union(num, num - 1));
        //     longest = std::cmp::max(longest, disjoint.union(num, num + 1));
        // }
        // return longest as i32;
    }

    pub fn longest_consecutive_with_set(nums: Vec<i32>) -> i32 {
        let sets = nums.into_iter().collect::<HashSet<i32>>();
        let mut count = 0;
        for &v in sets.iter() {
            if sets.contains(&(v - 1)) {
                continue;
            }
            let cnt = (v..).take_while(|x| sets.contains(x)).count();
            count = std::cmp::max(count, cnt);
        }
        count as i32
    }
}

#[test]
fn longest_consecutive_sequence_128_test() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );

    assert_eq!(Solution::longest_consecutive(vec![]), 0);
    assert_eq!(Solution::longest_consecutive(vec![1]), 1);
}
