pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut k = -1;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] {
                k = i as i32;
            }
        }

        if k == -1 {
            nums.sort();
            return;
        }
        let mut l = k + 1;

        for i in (k as usize + 1)..nums.len() {
            if nums[i] > nums[k as usize] {
                l = i as i32;
            }
        }
        nums.swap(k as usize, l as usize);

        let mut i = 0;
        let n = nums.len();
        while i as i32 + k + 1 < (n - 1 - i) as i32 {
            nums.swap(i + 1 + k  as usize, n - 1 - i);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        {
            let mut nums = vec![1, 2, 3];
            Solution::next_permutation(&mut nums);
            assert_eq!(nums, vec![1, 3, 2]);
        }
        {
            let mut nums = vec![3, 2, 1];
            Solution::next_permutation(&mut nums);
            assert_eq!(nums, vec![1, 2, 3]);
        }
        {
            let mut nums = vec![1, 1, 5];
            Solution::next_permutation(&mut nums);
            assert_eq!(nums, vec![1, 5, 1]);
        }
    }
}
