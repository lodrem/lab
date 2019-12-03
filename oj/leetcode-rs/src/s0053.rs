pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::solve_by_dp(nums)
    }

    // Greedy
    #[allow(dead_code)]
    pub fn solve_by_greedy(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];

            if sum > result {
                result = sum;
            }

            if sum < 0 {
                sum = 0;
            }
        }

        result
    }

    // Dynamic Programming
    // f(i) = (f(i - 1) > 0 ? f(i - 1) : 0) + A[i]
    #[allow(dead_code)]
    pub fn solve_by_dp(nums: Vec<i32>) -> i32 {
        let mut f = vec![0; nums.len()];
        let mut result = nums[0];

        f[0] = nums[0];

        for i in 1..nums.len() {
            if f[i - 1] > 0 {
                f[i] = f[i - 1] + nums[i];
            } else {
                f[i] = nums[i];
            }
            if f[i] > result {
                result = f[i];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        {
            let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
            let expect = 6;

            let actual = Solution::max_sub_array(nums);

            assert_eq!(actual, expect)
        }

        {
            let nums = vec![-2, 1];
            let expect = 1;

            let actual = Solution::max_sub_array(nums);

            assert_eq!(actual, expect)
        }
    }
}
