#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s.into_bytes();
        s.reverse();
        let mut stored_index = 0;
        let mut i = 0;
        while i < s.len() {
            if s[i] != ' ' as u8 {
                if stored_index != 0 {
                    s[stored_index] = ' ' as u8;
                    stored_index += 1;
                }
                let mut j = i;
                let b = stored_index;
                while j < s.len() && s[j] != ' ' as u8 {
                    s[stored_index] = s[j];
                    j += 1;
                    i += 1;
                    stored_index += 1;
                }
                s[b..stored_index].reverse();
            }
            i += 1;
        }
        s.truncate(stored_index);
        String::from_utf8(s).unwrap()
    }
}

#[test]
fn reverse_words_test1() {
    let s = "the sky is blue".to_string();
    assert_eq!(Solution::reverse_words(s), "blue is sky the".to_string());
}

#[test]
fn reverse_words_test2() {
    let s = "  hello world  ".to_string();
    assert_eq!(Solution::reverse_words(s), "world hello".to_string());
}

#[test]
fn reverse_words_test3() {
    let s = "  Bob    Loves  Alice   ".to_string();
    assert_eq!(Solution::reverse_words(s), "Alice Loves Bob".to_string());
}

#[test]
fn reverse_words_test4() {
    let s = "Alice does not even like bob".to_string();
    assert_eq!(
        Solution::reverse_words(s),
        "bob like even not does Alice".to_string()
    );
}
