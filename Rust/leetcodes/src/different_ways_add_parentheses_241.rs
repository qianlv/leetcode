#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        return Solution::diff_ways_to_compute_helper(expression.as_bytes());
    }

    fn is_operator(ch: u8) -> bool {
        return ch == '+' as u8 || ch == '-' as u8 || ch == '*' as u8;
    }

    fn to_int(exp: &[u8]) -> i32 {
        let mut ret = 0;
        let mut b = 0;
        let mut sign = 1;
        if exp[0] == '-' as u8 {
            sign = -1;
            b = 1;
        }

        for &ch in &exp[b..] {
            ret = ret * 10 + (ch as i32 - '0' as i32);
        }
        return sign * ret;
    }

    fn diff_ways_to_compute_helper(expression: &[u8]) -> Vec<i32> {
        let mut result = Vec::new();
        for (i, &ch) in expression.iter().enumerate() {
            if i > 0 && Solution::is_operator(ch) && !Solution::is_operator(expression[i - 1]) {
                let left = Solution::diff_ways_to_compute_helper(&expression[..i]);
                let right = Solution::diff_ways_to_compute_helper(&expression[i + 1..]);
                for &v in left.iter() {
                    for &u in right.iter() {
                        if ch == '+' as u8 {
                            result.push(v + u);
                        }
                        if ch == '-' as u8 {
                            result.push(v - u);
                        }
                        if ch == '*' as u8 {
                            result.push(v * u);
                        }
                    }
                }
            }
        }
        if result.is_empty() {
            result.push(Solution::to_int(expression));
        }
        return result;
    }
}

#[test]
fn diff_ways_to_compute_test() {
    assert_eq!(Solution::diff_ways_to_compute("2-1-1".to_string()), [2, 0]);
    assert_eq!(
        Solution::diff_ways_to_compute("2*3-4*5".to_string()),
        [-34, -10, -14, -10, 10]
    );
    assert_eq!(Solution::diff_ways_to_compute("2*-4".to_string()), [-8]);
}
