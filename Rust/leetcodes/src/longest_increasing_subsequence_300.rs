#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // dp[i] 表示长度为i的递增序列的最后一个值的最小值
        let mut dp = Vec::with_capacity(nums.len() + 1);
        dp.push(-10001);
        for v in nums {
            match dp.binary_search(&v) {
                Err(upper) => {
                    if upper == dp.len() {
                        dp.push(v);
                    } else {
                        dp[upper] = v;
                    }
                }
                _ => {}
            }
        }
        return dp.len() as i32 - 1;
    }
}

#[test]
fn longest_increasing_subsequence_300_test() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(Solution::length_of_lis(nums), 4);

    let nums = vec![0, 1, 0, 3, 2, 3];
    assert_eq!(Solution::length_of_lis(nums), 4);

    let nums = vec![7, 7, 7, 7, 7, 7];
    assert_eq!(Solution::length_of_lis(nums), 1);
}
