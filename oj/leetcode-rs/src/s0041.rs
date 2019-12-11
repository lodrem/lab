pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        {
            // if not contains 1
            // returns 1

            let mut contains_1 = false;
            for i in 0..nums.len() {
                if nums[i] == 1 {
                    contains_1 = true;
                } else if nums[i] < 1 {
                    nums[i] = 1;
                }
            }

            if !contains_1 {
                return 1;
            }
        }

        for i in 0..nums.len() {
            let idx = nums[i].abs() as usize;

            if idx > nums.len() || idx == 0 {
                continue;
            }

            let n = nums[idx - 1];

            if n > 0 {
                nums[idx - 1] = -n;
            }
        }

        for i in 0..nums.len() {
            if nums[i] >= 0 {
                return (i + 1) as i32;
            }
        }

        (nums.len() + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::first_missing_positive(vec![1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![2, 1]), 3);
        assert_eq!(Solution::first_missing_positive(vec![1, 2]), 3);
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
