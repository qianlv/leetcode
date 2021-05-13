#![allow(dead_code)]

struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Pair {
    x: i32,
    y: i32,
    yi: usize,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        return (self.x + self.y) == (other.x + other.y);
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Pair {}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = (self.x + self.y).cmp(&(other.x + other.y));
        match ord {
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for &x in nums1.iter() {
            heap.push(Pair {
                x,
                y: nums2[0],
                yi: 0,
            });
        }

        let mut ret = Vec::with_capacity(k as usize);

        while !heap.is_empty() && ret.len() < k as usize {
            let top = heap.pop().unwrap();
            ret.push(vec![top.x, top.y]);
            if top.yi + 1 < nums2.len() {
                heap.push(Pair {
                    x: top.x,
                    y: nums2[top.yi + 1],
                    yi: top.yi + 1,
                });
            }
        }
        return ret;
    }
}

#[test]
fn k_smallest_pairs_test() {
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec![vec![1, 2], vec![1, 4], vec![1, 6]]
    );

    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        vec![vec![1, 1], vec![1, 1]]
    );

    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
        vec![vec![1, 3], vec![2, 3]]
    );
}
