pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest_price = std::i32::MAX;
        let mut max_profit = 0;
        for i in 0..prices.len() {
            if prices[i] < lowest_price {
                lowest_price = prices[i];
            } else if prices[i] - lowest_price > max_profit {
                max_profit = prices[i] - lowest_price;
            }
        }
        return max_profit;
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(10, Solution::max_profit(vec![10, 20, 1, 2, 5, 10]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
