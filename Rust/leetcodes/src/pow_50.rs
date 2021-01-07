#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let x = if n < 0 { 1.0 / x } else { x };
        Solution::my_pow_helper(x, n)
    }

    fn my_pow_helper(x: f64, n: i32) -> f64 {
        match (x, n) {
            (_, 0) => 1.0,
            (x, n) => {
                let mut half = Solution::my_pow_helper(x, n / 2);
                half *= half;
                if n % 2 != 0 {
                    half *= x;
                }
                half
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn my_pow() {
        assert_approx_eq!(Solution::my_pow(1.0, -2147483648), 1.0);
        assert_approx_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_approx_eq!(Solution::my_pow(2.1, 3), 9.261);
        assert_approx_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}
