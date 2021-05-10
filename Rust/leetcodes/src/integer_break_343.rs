#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = Vec::new();
        dp.resize(n + 1, 0);
        dp[1] = 1;
        for i in 2..=n {
            if i != n {
                dp[i] = i;
            }
            for j in 1..i {
                dp[i] = std::cmp::max(dp[i], dp[j] * (i - j));
            }
            // println!("{} {}", i, dp[i]);
        }
        return dp[n] as i32;
    }
}

#[test]
fn integer_break_343_test() {
    assert_eq!(Solution::integer_break(2), 1);
    assert_eq!(Solution::integer_break(3), 2);
    assert_eq!(Solution::integer_break(10), 36);
}
