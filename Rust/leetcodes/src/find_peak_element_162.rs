#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0 as i32;
        }

        let mut low = 0 as usize;
        let mut high = nums.len();

        while high - low > 2 {
            let mid = (high - low) / 2 + low;
            if nums[mid] > nums[mid + 1] {
                high = mid + 1;
            } else {
                low = mid;
            }
        }

        if nums[low] > nums[low + 1] {
            low as i32
        } else {
            (low + 1) as i32
        }
    }
}

#[test]
fn find_peak_element_test1() {
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3]), 2);
    assert_eq!(Solution::find_peak_element(vec![3, 2, 1]), 0);
    assert_eq!(Solution::find_peak_element(vec![3, 2]), 0);
    assert_eq!(Solution::find_peak_element(vec![3]), 0);
}
