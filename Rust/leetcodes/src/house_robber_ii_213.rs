#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let f = |(pprev, prev), x| (prev, std::cmp::max(pprev + x, prev));
        let (_, result1) = nums.iter().skip(1).fold((0, 0), f);
        let (_, result2) = nums.iter().take(nums.len() - 1).fold((0, 0), f);
        std::cmp::max(result1, result2)
    }
}

#[test]
fn rob_ii_test1() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn rob_ii_test2() {
    assert_eq!(Solution::rob(vec![0]), 0);
}

#[test]
fn rob_ii_test3() {
    assert_eq!(Solution::rob(vec![9, 2, 1, 20]), 22);
}
