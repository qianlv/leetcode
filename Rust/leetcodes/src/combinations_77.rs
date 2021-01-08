#![allow(dead_code)]

struct Solution;

impl Solution {
    fn bitnum(value: i32) -> i32 {
        let mut num = 0i32;
        let mut value = value;
        while value > 0 {
            value = value & (value - 1);
            num += 1;
        }
        num
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut result: Vec<i32> = Vec::with_capacity(k as usize);
        let pow2 = 1 << n;
        for i in 0..pow2 {
            if Self::bitnum(i) == k {
                result.clear();
                for j in 0..n {
                    if (i & (1 << j)) > 0 {
                        result.push(j + 1);
                    }
                }
                results.push(result.clone());
            }
        }
        results
    }
}

#[test]
fn combine_test() {
    println!("{:?}", Solution::combine(4, 2));
    println!("{:?}", Solution::combine(1, 1));
    assert!(true);
}
