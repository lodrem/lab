pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        nums.sort();

        let mut result = vec![];

        for i in 0..nums.len() - 2 {
            let x = nums[i];

            if i > 0 && x == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let y = nums[j];
                let z = nums[k];

                let sum = x + y + z;

                if sum == 0 {
                    result.push(vec![x, y, z]);
                    j += 1;
                    k -= 1;

                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if sum > 0 {
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        {
            let nums = vec![-1, 0, 1, 2, -1, -4];
            let _expect = vec![vec![-1, 0, 1], vec![-1, -1, 2]];

            let _actual = Solution::three_sum(nums);
        }
    }
}
