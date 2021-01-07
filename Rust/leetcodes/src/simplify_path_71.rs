#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let tokens = path.split("/");
        let mut stack: Vec<&str> = Vec::new();
        stack.push("");
        for token in tokens {
            // println!("{}, {:?}", token, stack);
            match token {
                ".." => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                }
                "." => (),
                token => {
                    if !token.is_empty() {
                        stack.push(token);
                    }
                }
            }
        }
        if stack.len() == 1 {
            return "/".to_string();
        }
        stack.join("/")
    }
}

#[test]
fn simplify_path_tese() {
    assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    assert_eq!(
        Solution::simplify_path("/home//foo".to_string()),
        "/home/foo"
    );
    assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c");
    assert_eq!(
        Solution::simplify_path("/a/../../b/../c//.//".to_string()),
        "/c"
    );
}
