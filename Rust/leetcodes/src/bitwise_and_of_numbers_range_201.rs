#![allow(dead_code)]

struct Solution;
// 对于 And 只好出现对应bit出现0，那么数值对应位就是0了，
// 观察i32的31位bit的变化规律:
// 第0位: 0101010101010101 ... 2  循环
// 第1位: 0011001100110011 ... 4  循环
// 第2位: 0000111100001111 ... 8  循环
// 第3位: 0000000011111111 ... 16 循环
// ...
// 然后就是判断每一位,是否让 [m, n] 这个范围的所有值都落在全是 1 的范围
// 判断标准就是 m - n - 1 <= 循环长度 / 2, 同时 m, n 这对应bit位置都是1
//
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let n = n as u32;
        let m = m as u32;
        let range = (n - m + 1) as u32;
        let mut result = 0;
        for i in 0..31u32 {
            // println!("{} {} {}", range, (1 << i), ((m >> i) & 1));
            if range <= (1 << i) && ((m >> i) & 1) > 0 && ((n >> i) & 1) > 0 {
                result += 1 << i;
            }
        }
        result
    }
}

#[test]
fn range_bitwise_and_test() {
    // let t = ((1 as u32) << 31u32) - 1;
    // println!("{:?}", t);
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    assert_eq!(Solution::range_bitwise_and(1, 1), 1);
    assert_eq!(Solution::range_bitwise_and(1, 2), 0);
    assert_eq!(Solution::range_bitwise_and(3, 4), 0);
}
