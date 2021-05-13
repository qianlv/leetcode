#![allow(dead_code)]

struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct UglyNumber {
    value: i32,
    ugly_i: usize,
    prime_i: usize,
}

impl PartialEq for UglyNumber {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd for UglyNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for UglyNumber {}

impl Ord for UglyNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut uglys = Vec::with_capacity(primes.len());
        let mut heap = BinaryHeap::with_capacity(primes.len());
        uglys.push(1);
        for (i, &x) in primes.iter().enumerate() {
            heap.push(UglyNumber {
                value: x,
                ugly_i: 0,
                prime_i: i,
            });
        }
        while uglys.len() < n as usize {
            let top = heap.pop().unwrap();
            if uglys.last().unwrap() != &top.value {
                uglys.push(top.value);
            }
            let min_ugly = uglys[top.ugly_i + 1];
            heap.push(UglyNumber {
                value: primes[top.prime_i] * min_ugly,
                ugly_i: top.ugly_i + 1,
                prime_i: top.prime_i,
            });
        }
        return *uglys.last().unwrap();
    }
}

#[test]
fn super_ungly_number_313_test() {
    assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
}
