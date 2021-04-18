#![allow(dead_code)]
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut inheap = HashSet::new();
        heap.push(Reverse(1 as u64));
        inheap.insert(1 as u64);

        let mut i = 1;
        let mut ret = 1;
        while let Some(Reverse(v)) = heap.pop() {
            if i == n {
                ret = v;
                break;
            }
            println!("{} {}", i, v);
            if !inheap.contains(&(v * 2)) {
                heap.push(Reverse(v * 2));
                inheap.insert(v * 2);
            }
            if !inheap.contains(&(v * 3)) {
                heap.push(Reverse(v * 3));
                inheap.insert(v * 3);
            }
            if !inheap.contains(&(v * 5)) {
                heap.push(Reverse(v * 5));
                inheap.insert(v * 5);
            }
            i += 1;
        }
        return ret as i32;
    }
}

#[test]
fn nth_ugly_number_test() {
    assert_eq!(Solution::nth_ugly_number(10), 12);
    assert_eq!(Solution::nth_ugly_number(1407), 536870912);
}
