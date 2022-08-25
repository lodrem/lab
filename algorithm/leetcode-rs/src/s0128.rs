pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let set: HashSet<i32> = nums.into_iter().collect();

        let mut res = 0;

        for num in set.iter() {
            if !set.contains(&(num - 1)) {
                let mut cur = *num;
                let mut len = 1;

                while set.contains(&(cur + 1)) {
                    cur += 1;
                    len += 1;
                }

                res = std::cmp::max(res, len);
            }
        }

        res
    }
}
