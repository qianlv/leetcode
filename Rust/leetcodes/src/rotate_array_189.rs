#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() <= 1 {
            return;
        }

        let k = k as usize % nums.len();
        let m = nums.len() - k as usize;
        Solution::reverse(nums, 0, m);
        Solution::reverse(nums, m, nums.len());
        Solution::reverse(nums, 0, nums.len());
    }

    pub fn reverse(nums: &mut Vec<i32>, b: usize, e: usize) {
        if b + 1 >= e {
            return;
        }

        let mut i = b;
        let mut j = e - 1;
        while i < j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[test]
fn rotate_array_189_test() {
    let mut nums = vec![1, 2];
    Solution::rotate(&mut nums, 0);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![1, 2];
    Solution::rotate(&mut nums, 1);
    assert_eq!(nums, vec![2, 1]);

    let mut nums = vec![1, 2];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![6, 7, 1, 2, 3, 4, 5]);
}
