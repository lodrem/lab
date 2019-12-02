pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // f[i] = max(f[i-1], p[i] + f[i-2])

        if nums.is_empty() {
            return 0;
        }

        let mut f = vec![0; nums.len()];

        f[0] = nums[0];

        for i in 1..nums.len() {
            let p = if i >= 2 { f[i - 2] } else { 0 };

            f[i] = std::cmp::max(f[i - 1], nums[i] + p);
        }

        f[f.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
