#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut dna_sequences: HashMap<&str, u32> = HashMap::new();
        let mut ret = Vec::new();
        for i in 10..=s.len() {
            let counter = dna_sequences.entry(&s[i - 10..i]).or_insert(0);
            *counter += 1;
            if *counter == 2 {
                ret.push(s[i - 10..i].to_string());
            }
        }
        ret
    }
}

#[test]
fn find_repeated_dna_sequences_test1() {
    assert_eq!(
        Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()),
        vec!["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()]
    );
}

#[test]
fn find_repeated_dna_sequences_test2() {
    assert_eq!(
        Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
        vec!["AAAAAAAAAA".to_string()]
    );
}

#[test]
fn find_repeated_dna_sequences_test3() {
    assert_eq!(
        Solution::find_repeated_dna_sequences("AAAAAAAAAAA".to_string()),
        vec!["AAAAAAAAAA".to_string()]
    );
}
