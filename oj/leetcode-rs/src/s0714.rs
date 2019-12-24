pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut profit = 0;
        let mut hold = -prices[0];
        for i in 1..prices.len() {
            let price = prices[i];

            // sell
            profit = std::cmp::max(profit, hold + price - fee);

            // buy
            hold = std::cmp::max(hold, profit - price);
        }

        profit
    }
}
