#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        a.into_iter()
            .scan(0, |state, x| {
                *state = (*state * 2 + x) % 5;
                Some(*state == 0)
            })
            .collect()
    }
}
