pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp;
        let n = nums.len();

        if k == 1 {
            return nums;
        } else if k == 0 {
            return vec![];
        }
        let k = k as usize;

        let mut left = vec![0; n];
        let mut right = vec![0; n];
        left[0] = nums[0];
        right[n - 1] = nums[n - 1];

        for i in 1..n {
            left[i] = if i % k == 0 {
                nums[i]
            } else {
                cmp::max(left[i - 1], nums[i])
            };

            let j = n - i - 1;
            right[j] = if (j + 1) % k == 0 {
                nums[j]
            } else {
                cmp::max(right[j + 1], nums[j])
            };
        }

        let mut res = Vec::with_capacity(n - k + 1);
        for i in 0..n - k + 1 {
            res.push(cmp::max(left[i + k - 1], right[i]));
        }
        res
    }
}
