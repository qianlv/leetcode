#![allow(dead_code)]

struct Solution;

struct LagerPositive {
    value: Vec<i32>,
}

impl LagerPositive {
    fn new(value: Vec<i32>) -> LagerPositive {
        LagerPositive { value }
    }

    fn divide2(&mut self) {
        let mut pre = 0;
        for v in self.value.iter_mut().rev() {
            let value = pre * 10 + *v;
            *v = value / 2;
            pre = value & 0x1;
        }
        // println!("{:?}", self.value);
        while let Some(v) = self.value.last() {
            if v == &0 {
                self.value.pop();
            } else {
                break;
            }
        }
    }

    fn is_zero(&self) -> bool {
        return self.value.len() == 0;
    }

    fn is_odd(&self) -> bool {
        return (self.value.first().unwrap() & 0x1) == 1;
    }
}

impl Solution {
    const MOD: i32 = 1337;
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut b = b;
        b.reverse();
        let mut b = LagerPositive::new(b);
        let mut powera = a % Solution::MOD;
        let mut ret = 1;
        while !b.is_zero() {
            if b.is_odd() {
                ret = (ret * powera) % Solution::MOD;
            }
            powera = (powera * powera) % Solution::MOD;
            b.divide2();
            // println!("{:?}", b.value);
        }
        ret
    }
}

#[test]
fn super_pow_372_largest_positive_test1() {
    let mut lp = LagerPositive::new(vec![0, 0, 2]);
    lp.divide2();
    assert_eq!(lp.value, vec![0, 0, 1]);
    lp.divide2();
    assert_eq!(lp.value, vec![0, 5]);
    lp.divide2();
    assert_eq!(lp.value, vec![5, 2]);
    lp.divide2();
    assert_eq!(lp.value, vec![2, 1]);
    lp.divide2();
    assert_eq!(lp.value, vec![6]);
    lp.divide2();
    assert_eq!(lp.value, vec![3]);
    lp.divide2();
    assert_eq!(lp.value, vec![1]);
    lp.divide2();
    assert_eq!(lp.value, vec![]);
}

#[test]
fn super_pow_372_largest_positive_test2() {
    let mut lp = LagerPositive::new(vec![2, 5, 8, 3, 3, 4]);
    lp.divide2();
    assert_eq!(lp.value, vec![6, 2, 9, 6, 1, 2]);
    lp.divide2();
    assert_eq!(lp.value, vec![3, 6, 4, 8, 0, 1]);
    lp.divide2();
    assert_eq!(lp.value, vec![1, 3, 2, 4, 5]);
    lp.divide2();
    assert_eq!(lp.value, vec![5, 1, 1, 7, 2]);
    lp.divide2();
    assert_eq!(lp.value, vec![7, 5, 5, 3, 1]);
    lp.divide2();
    assert_eq!(lp.value, vec![8, 7, 7, 6]);
    lp.divide2();
    assert_eq!(lp.value, vec![9, 8, 3, 3]);
}

#[test]
fn super_pow_372_test() {
    assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
    assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
    assert_eq!(Solution::super_pow(2, vec![3]), 8);
}
