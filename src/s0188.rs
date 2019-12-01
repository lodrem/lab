pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(max_transaction: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let max_transaction = max_transaction as usize;

        if max_transaction > prices.len() / 2 {
            return Self::max_profit_without_limit(prices);
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

    fn max_profit_without_limit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        let mut idx = 0;

        while idx < prices.len() {
            let lowest_price = prices[idx];

            while idx + 1 < prices.len() && prices[idx] <= prices[idx + 1] {
                idx += 1;
            }

            profit += prices[idx] - lowest_price;
            idx += 1;
        }

        profit
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
        assert_eq!(7, Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
    }
}
