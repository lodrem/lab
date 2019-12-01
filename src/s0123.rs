pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // dp[i] = max(dp[i-1], prices[i] - prices[j] + dp[j-2]), j=[0..i-1]

        let mut dp = vec![0; prices.len()];

        for i in 1..prices.len() {
        }

        use std::cmp::max;

        let mut hold1 = std::i32::MIN;
        let mut hold2 = std::i32::MIN;
        let mut release1 = 0;
        let mut release2 = 0;
        for price in prices {
            release2 = max(release2, hold2 + price);
            hold2 = max(hold2, release1 - price);
            release1 = max(release1, hold1 + price);
            hold1 = max(hold1, -price);
        }
        release2
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
