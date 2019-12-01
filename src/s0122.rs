pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
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
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
