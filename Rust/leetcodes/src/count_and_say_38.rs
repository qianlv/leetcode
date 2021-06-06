#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        if n == 2 {
            return "11".to_string();
        }
        let s = Solution::count_and_say(n - 1);
        let s = s.as_bytes();
        let len = s.len();
        let mut cnt = 1;
        let mut ret: String = String::new();
        for (prev, curr) in s[..len - 1].iter().zip(s[1..len].iter()) {
            if prev == curr {
                cnt += 1;
            } else {
                ret.push_str(cnt.to_string().as_str());
                ret.push(*prev as char);
                cnt = 1;
            }
        }

        ret.push_str(cnt.to_string().as_str());
        ret.push(s[len - 1] as char);
        return ret;
    }
}

#[test]
fn count_and_say_38_test() {
    assert_eq!(Solution::count_and_say(4), "1211".to_string());
}
