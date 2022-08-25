pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let nums = {
            let mut n = vec![0; nums.len() + 2];
            for i in 0..nums.len() {
                n[i + 1] = nums[i];
            }
            n[0] = 1;
            n[nums.len() + 1] = 1;

            n
        };

        // f[i][j] = max(f[i][k-1] + nums[i-1] * nums[k] * nums[j+1] + f[k+1][j]), i <= k <= j
        let mut f = vec![vec![0; nums.len()]; nums.len()];

        for right in 2..nums.len() {
            for left in (0..right - 1).rev() {
                for i in left + 1..right {
                    f[left][right] = std::cmp::max(
                        f[left][right],
                        nums[left] * nums[i] * nums[right] + f[left][i] + f[i][right],
                    )
                }
            }
        }

        f[0][f.len() - 1]
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
