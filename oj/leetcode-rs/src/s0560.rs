pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        // => sum[i..j] = k
        // sum[i..j] = sum[0..j] - sum[0..i]
        // sum[0..i] = sum[0..j] - k

        // Store sum[0..j]
        let mut sum = 0;
        let mut count = 0;
        // Store sum[0..i] => count(i)
        let mut map = HashMap::new();

        // Implicit:
        // sum[0..0] == 0
        // => sum[0..j] == k
        map.insert(0, 1);

        for j in 0..nums.len() {
            sum += nums[j];
            if let Some(v) = map.get(&(sum - k)) {
                count += v;
            }
            *map.entry(sum).or_default() += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::subarray_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0),
            55
        );
        assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
