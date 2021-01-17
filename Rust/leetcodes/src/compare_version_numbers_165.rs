#![allow(dead_code)]

struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1_vecs = version1.split(".").collect::<Vec<&str>>();
        let mut v2_vecs = version2.split(".").collect::<Vec<&str>>();
        if v1_vecs.len() < v2_vecs.len() {
            v1_vecs.extend(std::iter::repeat("0").take(v2_vecs.len() - v1_vecs.len()));
        } else {
            v2_vecs.extend(std::iter::repeat("0").take(v1_vecs.len() - v2_vecs.len()));
        }

        let iter1 = v1_vecs.iter().map(|x| x.parse::<i32>().unwrap());
        let iter2 = v2_vecs.iter().map(|x| x.parse::<i32>().unwrap());
        match iter1.cmp(iter2) {
            Ordering::Less => return -1,
            Ordering::Equal => return 0,
            Ordering::Greater => return 1,
        }
    }
}

#[test]
fn compare_version_test1() {
    assert_eq!(
        Solution::compare_version("1.01".to_string(), "1.001".to_string()),
        0
    );
}

#[test]
fn compare_version_test2() {
    assert_eq!(
        Solution::compare_version("1.0".to_string(), "1.0.0".to_string()),
        0
    );
}

#[test]
fn compare_version_test3() {
    assert_eq!(
        Solution::compare_version("0.1".to_string(), "1.1".to_string()),
        -1
    );
}

#[test]
fn compare_version_test4() {
    assert_eq!(
        Solution::compare_version("1.0.1".to_string(), "1".to_string()),
        1
    );
}

#[test]
fn compare_version_test5() {
    assert_eq!(
        Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string()),
        -1
    );
}

#[test]
fn compare_version_test6() {
    assert_eq!(
        Solution::compare_version("1.2".to_string(), "1.10".to_string()),
        -1
    );
}
