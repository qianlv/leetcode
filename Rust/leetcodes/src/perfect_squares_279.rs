#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut values = Vec::new();
        values.resize((n + 1) as usize, n);
        let mut i = 1usize;
        while i * i <= n as usize {
            values[i * i] = 1;
            i += 1;
        }

        for i in 2..=n as usize {
            // println!("{}:", i);
            for j in 1..=i {
                if j * j > i {
                    break;
                }
                values[i] = std::cmp::min(values[i], 1 + values[i - j * j]);
                // println!("{} {}", j, i - j);
            }
            // println!("{}", values[i]);
        }
        return values[n as usize];
    }
}

#[test]
fn perfect_squares_279_test() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}
