pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn by_hash_map(nums: Vec<i32>) -> i32 {
        // time: O(n)
        // space: O(n)

        use std::collections::HashMap;

        let mut count = HashMap::new();
        let threshold = nums.len() / 2;

        for num in nums.into_iter() {
            let c = count.entry(num).or_insert(0);
            *c += 1;

            if *c > threshold {
                return num;
            }
        }
        0
    }

    fn by_boyer_moore_voting(nums: Vec<i32>) -> i32 {
        // time: O(n)
        // space: O(1)

        let mut candidate = 0;
        let mut count = 0;

        for num in nums.into_iter() {
            if count == 0 {
                candidate = num;
            }

            count += if candidate == num { 1 } else { -1 };
        }

        candidate
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        Self::by_boyer_moore_voting(nums)
    }
}
