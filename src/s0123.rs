pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn max_profit_with_max_transaction(max_transaction: usize, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        // dp[k, i] = max(dp[k, i-1], prices[i] - prices[j] + dp[k-1, j-1]), j=[1..i]

        let mut dp = vec![vec![0; prices.len()]; max_transaction + 1];

        for k in 1..dp.len() {
            let mut min = prices[0];
            for i in 1..prices.len() {
                min = std::cmp::min(min, prices[i] - dp[k - 1][i - 1]);
                dp[k][i] = std::cmp::max(dp[k][i - 1], prices[i] - min);
            }
        }

        dp[max_transaction][prices.len() - 1]
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        const MAX_TRANSACTION: usize = 2;

        // calculate max profit with max k transactions
        Self::max_profit_with_max_transaction(MAX_TRANSACTION, prices)
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
