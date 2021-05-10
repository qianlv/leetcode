#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a as u32;
        let mut b = b as u32;
        let mut ret = 0;
        let mut flag = 0;
        let mut power = 1;
        while a != 0 || b != 0 || flag != 0 {
            match (a & 0x1, b & 0x1, flag) {
                (1, 0, 0) | (0, 1, 0) | (0, 0, 1) => {
                    ret |= power;
                    flag = 0;
                }
                (1, 1, 1) => {
                    ret |= power;
                }
                (0, 0, 0) => {}
                _ => {
                    flag = 1;
                }
            }
            // eprintln!("{} {} {} {} {} {}", a, b, a & 0x1, b & 0x1, power, ret);
            a >>= 1;
            b >>= 1;
            power <<= 1;
        }
        ret
    }
}

#[test]
fn sum_of_two_integers_371_test() {
    assert_eq!(Solution::get_sum(2, 3), 5);
    assert_eq!(Solution::get_sum(-1, 1), 0);
    assert_eq!(Solution::get_sum(-100, 101), 1);
    assert_eq!(Solution::get_sum(20, 100), 120);
    assert_eq!(Solution::get_sum(120, -100), 20);
}
