#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut sets = HashSet::new();
        let mut result = Vec::new();
        result.push(0);
        sets.insert(0);
        Self::search(&mut sets, &mut result, 0, n);
        result
    }

    fn search(sets: &mut HashSet<i32>, result: &mut Vec<i32>, code: i32, n: i32) {
        for i in 0..n {
            let next_code = code ^ (1 << i);
            if !sets.contains(&next_code) {
                sets.insert(next_code);
                result.push(next_code);
                Self::search(sets, result, next_code, n);
                break;
            }
        }
    }
}

#[test]
fn gray_code_test() {
    let codes = Solution::gray_code(2);
    println!("{:?}", codes);
}
