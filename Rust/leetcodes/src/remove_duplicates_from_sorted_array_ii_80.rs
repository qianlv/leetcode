#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut first = 0;
        let mut second = 1;
        for i in 2..nums.len() {
            if nums[i] != nums[first] || nums[i] != nums[second] {
                nums[second + 1] = nums[i];
                first += 1;
                second += 1;
            }
        }
        nums.truncate(second + 1);
        nums.len() as i32
    }
}

#[test]
fn remove_duplicates_test1() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    println!("{:?}", nums);
}

#[test]
fn remove_duplicates_test2() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    assert_eq!(Solution::remove_duplicates(&mut nums), 7);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
}
