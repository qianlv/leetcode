#![allow(dead_code)]

struct Solution;

// 二分枚举长度，然后O(n) 判断这个长度是否有连续subarray of sum >= s.
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut high = nums.len() + 1;
        while low < high {
            let mid = (high - low) / 2 + low;
            if Self::greater_s_len(&nums, mid, s) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        if low == nums.len() + 1 {
            return 0;
        }
        return low as i32;
    }

    fn greater_s_len(nums: &Vec<i32>, len: usize, s: i32) -> bool {
        let mut sum: i32 = nums.iter().take(len).sum();
        if sum >= s {
            return true;
        }
        nums.iter().skip(len).zip(nums.iter()).any(|(v1, v2)| {
            sum += v1;
            sum -= v2;
            sum >= s
        })
    }
}

#[test]
fn min_sub_array_len_logn_test1() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}

#[test]
fn min_sub_array_len_logn_test2() {
    assert_eq!(Solution::min_sub_array_len(3, vec![1, 1]), 0);
}
