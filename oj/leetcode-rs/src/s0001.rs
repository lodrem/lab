pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut num_to_idx = HashMap::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;

            match num_to_idx.get(&diff) {
                Some(j) => return vec![*j, i as i32],
                None => num_to_idx.insert(num, i as i32),
            };
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
