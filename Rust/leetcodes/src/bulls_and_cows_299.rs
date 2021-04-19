#![allow(dead_code)]

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut counters = HashMap::new();
        for ch in guess.chars() {
            let counter = counters.entry(ch).or_insert(0);
            *counter += 1;
        }
        let mut correct_pos_count = 0;
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                correct_pos_count += 1;
                if let Some(counter) = counters.get_mut(&s) {
                    *counter -= 1;
                }
            }
        }

        let mut correct_char_count = 0;
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                continue;
            }
            if let Some(counter) = counters.get_mut(&s) {
                if (*counter) > 0 {
                    *counter -= 1;
                    correct_char_count += 1;
                }
            }
        }
        return format!("{}A{}B", correct_pos_count, correct_char_count);
    }
}

#[test]
fn bulls_and_cows_299_test() {
    assert_eq!(
        Solution::get_hint("1807".to_string(), "7810".to_string()),
        "1A3B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1123".to_string(), "0111".to_string()),
        "1A1B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1123".to_string(), "0100".to_string()),
        "1A0B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1123".to_string(), "0010".to_string()),
        "0A1B".to_string()
    );
}
