pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut f1 = vec![0; nums.len()];
        let mut f2 = vec![0; nums.len()];

        f1[0] = nums[0];
        if nums.len() > 1 {
            f2[1] = nums[1];
        }

        for i in 1..nums.len() - 1 {
            let p1 = if i >= 2 { f1[i - 2] } else { 0 };
            f1[i] = std::cmp::max(f1[i - 1], nums[i] + p1);
        }
        if nums.len() > 1 {
            f1[nums.len() - 1] = f1[nums.len() - 2];
        }

        for i in 2..nums.len() {
            let p2 = if i >= 2 { f2[i - 2] } else { 0 };
            f2[i] = std::cmp::max(f2[i - 1], nums[i] + p2);
        }

        std::cmp::max(f1[f1.len() - 1], f2[f2.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }
}
