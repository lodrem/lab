pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut jumps = 0;
        let mut farest_idx = 0;
        let mut next_jump_idx = 0;

        for i in 0..n - 1 {
            farest_idx = std::cmp::max(farest_idx, i + nums[i] as usize);

            if i == next_jump_idx {
                jumps += 1;
                next_jump_idx = farest_idx;
            }
        }

        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
