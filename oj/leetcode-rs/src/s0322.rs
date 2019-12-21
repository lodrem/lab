pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // f[i] = min{f[i], f[i - coins[j]] + 1 | j -> 0..coins.len}

        let amount = amount as usize;
        let mut f = vec![amount + 1; amount + 1];

        f[0] = 0;

        for i in 1..=amount {
            for j in 0..coins.len() {
                let coin = coins[j] as usize;
                if i >= coin {
                    f[i] = std::cmp::min(f[i], f[i - coin] + 1);
                }
            }
        }

        if f[amount] > amount {
            -1
        } else {
            f[amount] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }
}
