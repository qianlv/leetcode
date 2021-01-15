#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in tokens.iter() {
            match token.as_str() {
                "+" => {
                    let r = stack.pop().unwrap();
                    let l = stack.pop().unwrap();
                    stack.push(l + r);
                }
                "-" => {
                    let r = stack.pop().unwrap();
                    let l = stack.pop().unwrap();
                    stack.push(l - r);
                }
                "*" => {
                    let r = stack.pop().unwrap();
                    let l = stack.pop().unwrap();
                    stack.push(l * r);
                }
                "/" => {
                    let r = stack.pop().unwrap();
                    let l = stack.pop().unwrap();
                    stack.push(l / r);
                }
                x => {
                    stack.push(x.parse::<i32>().unwrap());
                }
            }
            // println!("{:?}", stack);
        }
        stack.pop().unwrap()
    }
}

#[test]
fn eval_rpn_test1() {
    let ret = Solution::eval_rpn(
        vec!["2", "1", "+", "3", "*"]
            .into_iter()
            .map(|x| x.to_owned())
            .collect(),
    );

    assert_eq!(ret, 9);
}

#[test]
fn eval_rpn_test2() {
    let ret = Solution::eval_rpn(
        vec!["4", "13", "5", "/", "+"]
            .into_iter()
            .map(|x| x.to_owned())
            .collect(),
    );

    assert_eq!(ret, 6);
}

#[test]
fn eval_rpn_test3() {
    let ret = Solution::eval_rpn(
        vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect(),
    );

    assert_eq!(ret, 22);
}
