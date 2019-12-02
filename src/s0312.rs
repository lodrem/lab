pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        // nums[left] * nums[i] * nums[right] + dp(left, i) + dp(i, right)
        // f[i] = max(f[i-1], p[i-1] * p[i] * p[i+1] + f[i-2])

        if nums.len() <= 3 {
            let mut coins = 1;
            for i in 0..nums.len() {
                coins *= nums[i];
            }
            return coins;
        }

        let mut f = vec![0; nums.len()];

        f[0] = nums[0] * nums[1];
        f[1] = nums[0] * nums[1] * nums[2];

        for i in 2..nums.len() {
            let next = if i + 1 < nums.len() { nums[i + 1] } else { 1 };
            let p = nums[i - 1] * nums[i] * next;

            f[i] = std::cmp::max(f[i - 1], p + f[i - 2]);
        }

        f[f.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(167, Solution::max_coins(vec![3, 1, 5, 8]));
    }
}
