struct Solution;
use std::collections::HashMap;

impl Solution {
    fn permute_recursive(
        nums_count: &mut HashMap<i32, i32>,
        keys: &Vec<i32>,
        results: &mut Vec<Vec<i32>>,
        result: &mut Vec<i32>,
    ) {
        let mut zero = true;
        for key in keys.iter() {
            // println!("key: {}", key);
            if let Some(val) = nums_count.get_mut(key) {
                if *val > 0 {
                    *val -= 1;
                    result.push(*key);
                    Self::permute_recursive(nums_count, keys, results, result);
                    result.pop();
                    *nums_count.get_mut(key).unwrap() += 1;
                    zero = false;
                }
            }
        }

        if zero {
            results.push(result.clone());
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_count = HashMap::new();
        nums.iter().for_each(|x| {
            let counter = nums_count.entry(*x).or_insert(0);
            *counter += 1;
        });

        let nums = nums_count.iter().map(|(key, _)| *key).collect();

        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut result = Vec::new();
        Self::permute_recursive(&mut nums_count, &nums, &mut results, &mut result);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn permute_unique() {
        let results = Solution::permute_unique(vec![1, 1, 2]);
        for ret in results.iter() {
            println!("{:?}", ret);
        }
    }
}
