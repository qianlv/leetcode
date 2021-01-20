#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut i = 0;
        let mut ret = nums
            .iter()
            .enumerate()
            .fold(nums.len() as i32 + 1, |min_len, (j, val)| {
                sum += val;
                while i < j && sum - nums[i] >= s {
                    sum -= nums[i];
                    i += 1;
                }
                if sum >= s {
                    // println!("{} {} {}", sum, i, j);
                    let len = (j - i + 1) as i32;
                    return std::cmp::min(len, min_len);
                }
                min_len
            });

        if ret == (nums.len() + 1) as i32 {
            ret = 0;
        }
        ret
    }
}

#[test]
fn min_sub_array_len_test1() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}

#[test]
fn min_sub_array_len_test2() {
    assert_eq!(Solution::min_sub_array_len(3, vec![1, 1]), 0);
}
