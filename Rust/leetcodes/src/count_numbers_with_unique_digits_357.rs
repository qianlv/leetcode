#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut count = 1;
        for i in 1..=n {
            let mut cnt = 9;
            for j in 1..i {
                cnt = cnt * (10 - j);
            }
            count += cnt;
        }
        count
    }
}

#[test]
fn count_numbers_with_unique_digits_test() {
    assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    assert_eq!(Solution::count_numbers_with_unique_digits(1), 10);
    assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
}
