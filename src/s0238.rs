pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut results = vec![1; nums.len()];

        for i in 1..nums.len() {
            results[i] = nums[i - 1] * results[i - 1];
        }

        let mut r = 1;
        for i in (0..nums.len()).rev() {
            results[i] *= r;
            r *= nums[i];
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
        assert_eq!(vec![0, 0], Solution::product_except_self(vec![0, 0]));
        assert_eq!(vec![1, 0], Solution::product_except_self(vec![0, 1]));
    }
}
