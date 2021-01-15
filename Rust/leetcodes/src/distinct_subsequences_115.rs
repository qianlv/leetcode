#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        if t.is_empty() {
            return 1;
        }

        let mut dp: [Vec<i32>; 2] = [Vec::new(), Vec::new()];
        dp[0].resize(s.len() + 1, 0);
        dp[1].resize(s.len() + 1, 0);

        let mut switch = 0;
        let t = t.as_bytes();
        let s = s.as_bytes();
        for (i, ch) in s.iter().enumerate() {
            if *ch == t[0] {
                dp[switch][i + 1] = dp[switch][i] + 1;
            } else {
                dp[switch][i + 1] = dp[switch][i];
            }
            // print!("{} ", dp[switch][i + 1]);
        }
        // println!("");
        for i in 1..t.len() {
            for j in 0..s.len() {
                if s[j] == t[i] {
                    // println!("{}, {}", t[i], s[j]);
                    dp[1 - switch][j + 1] = dp[1 - switch][j] + dp[switch][j];
                } else {
                    dp[1 - switch][j + 1] = dp[1 - switch][j]
                }
                // print!("{} ", dp[1 - switch][j + 1]);
            }
            // println!("");
            switch = 1 - switch;
        }
        dp[switch][s.len()]
    }
}

#[test]
fn num_distinct_test1() {
    assert_eq!(
        Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
        5
    );
}

#[test]
fn num_distinct_test2() {
    assert_eq!(
        Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
        3
    );
}
