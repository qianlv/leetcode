#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn sortword(s: &str) -> String {
        let mut s: Vec<u8> = s.bytes().into_iter().collect();
        s.sort();
        String::from_utf8(s).unwrap()
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ret: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let key = Solution::sortword(s.as_str());
            let list = ret.entry(key).or_insert(Vec::new());
            list.push(s);
        }
        ret.into_iter().map(|(_, s)| s).collect()
    }
}

#[test]
fn group_anagrams_test() {
    let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let ret = Solution::group_anagrams(strs);
    println!("{:?}", ret);

    let strs: Vec<String> = vec![""].into_iter().map(|s| s.to_string()).collect();
    let ret = Solution::group_anagrams(strs);
    println!("{:?}", ret);
}
