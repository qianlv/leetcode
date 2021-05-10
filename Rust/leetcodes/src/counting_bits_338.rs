#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut count: Vec<i32> = Vec::with_capacity(num as usize + 1);
        count.push(0);
        let num = num as usize;
        for i in 1..=num {
            // println!("{} {} {}", i, num, i >> 1);
            let mut cnt = 0;
            cnt += i & 0x1;
            cnt += count[i >> 1] as usize;
            count.push(cnt as i32);
        }
        count
    }
}

#[test]
fn test_counting_bits_338() {
    assert_eq!(Solution::count_bits(0), vec![0]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
