#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut pprev = 0;
        nums.iter().fold(0, |prev, x| {
            let ret = std::cmp::max(prev, pprev + x);
            pprev = prev;
            ret
        })
    }
}

#[test]
fn rob_test1() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn rob_test2() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
fn rob_test3() {
    assert_eq!(Solution::rob(vec![9, 2, 1, 20]), 29);
}
