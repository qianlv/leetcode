#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut palindrome = Vec::new();
        let mut palindromes = Vec::new();
        Solution::partition_helper(s.as_str(), &mut palindrome, &mut palindromes);
        palindromes
    }

    fn partition_helper<'a>(
        s: &'a str,
        palindrome: &mut Vec<&'a str>,
        palindromes: &mut Vec<Vec<String>>,
    ) {
        if s.is_empty() {
            if !palindrome.is_empty() {
                palindromes.push(palindrome.iter().map(|&x| x.to_string()).collect());
            }
            return;
        }
        for i in 0..=s.len() {
            if Solution::is_palindrome(&s[..i]) {
                palindrome.push(&s[..i]);
                Solution::partition_helper(&s[i..], palindrome, palindromes);
                palindrome.pop();
            }
        }
    }
    fn is_palindrome(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let len = s.len();
        let s = s.as_bytes();
        for i in 0..=len / 2 {
            if s[i] != s[len - i - 1] {
                return false;
            }
        }

        return true;
    }
}

#[test]
fn palindrome_partitioning_131_test() {
    assert_eq!(
        Solution::partition("aab".to_string()),
        vec![vec!["a", "a", "b"], vec!["aa", "b"]]
    );

    assert_eq!(Solution::partition("a".to_string()), vec![vec!["a"]]);
}
