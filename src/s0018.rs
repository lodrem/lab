pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums: Vec<i32> = nums.into_iter().collect();
        nums.sort();
        Self::n_sum(&nums[0..], target, 4)
    }

    pub fn n_sum(nums: &[i32], target: i32, n: usize) -> Vec<Vec<i32>> {
        if nums.len() < n
            || n < 2
            || target < nums[0] * n as i32
            || target > nums.last().unwrap() * n as i32
        {
            return vec![];
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut result = vec![];
        if n > 2 {
            while l < r {
                let sub_result = Self::n_sum(&nums[l + 1..], target - nums[l], n - 1);
                if sub_result.len() > 0 {
                    sub_result.into_iter().for_each(|mut v| {
                        v.push(nums[l]);
                        result.push(v);
                    });
                    while l + 1 < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                }
                l += 1;
            }
        } else if n == 2 {
            while l < r {
                let sum = nums[l] + nums[r];
                if sum < target {
                    l += 1;
                    continue;
                }
                if sum > target {
                    r -= 1;
                    continue;
                }
                result.push(vec![nums[r], nums[l]]);
                l += 1;
                r -= 1;
                while l < r && nums[l] == nums[l - 1] {
                    l += 1;
                }
                while l < r && nums[r] == nums[r + 1] {
                    r -= 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        {
            let actual = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
            let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];

            assert_eq!(expected, actual);
        }
        {
            let actual = Solution::four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11);
            let expected = vec![vec![-5, -4, -3, 1]];
            assert_eq!(expected, actual);
        }
    }
}
