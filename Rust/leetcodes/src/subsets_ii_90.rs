#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut uniq = nums.clone();
        uniq.dedup();
        let mut count = Vec::with_capacity(uniq.len());
        let mut i = 0;
        for &v in uniq.iter() {
            let mut cnt = 0;
            while i < nums.len() && nums[i] == v {
                cnt += 1;
                i += 1;
            }
            count.push(cnt);
        }
        // println!("{:?} {:?}", uniq, count);
        let mut sub = vec![];
        let mut subs = vec![];
        Solution::subsets_with_dup_helper(&uniq, &&count, 0, &mut sub, &mut subs);
        return subs;
    }

    fn subsets_with_dup_helper(
        uniq: &Vec<i32>,
        count: &Vec<i32>,
        index: usize,
        sub: &mut Vec<i32>,
        subs: &mut Vec<Vec<i32>>,
    ) {
        if index as usize == uniq.len() {
            subs.push(sub.clone());
            return;
        }
        // println!("{}", index);

        for i in 0..=count[index] {
            let old_len = sub.len();
            sub.resize(old_len + i as usize, uniq[index]);
            Solution::subsets_with_dup_helper(uniq, count, index + 1, sub, subs);
            sub.resize(old_len, 0);
        }
    }
}

#[test]
fn subsets_ii_90_test() {
    assert_eq!(
        Solution::subsets_with_dup(vec![1, 2, 2]),
        vec![
            vec![],
            vec![2],
            vec![2, 2],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2]
        ]
    );
}
