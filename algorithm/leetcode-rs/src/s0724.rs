pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = {
            let mut sum = 0;
            for i in 0..nums.len() {
                sum += nums[i];
            }
            sum
        };

        let mut left_sum = 0;

        for i in 0..nums.len() {
            if left_sum == (sum - left_sum - nums[i]) {
                return i as i32;
            }
            left_sum += nums[i];
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::pivot_index(vec![-1, -1, -1, 0, 1, 1]), 0);
        assert_eq!(Solution::pivot_index(vec![-1, -1, 0, 1, 1, 0]), 5);
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }
}
