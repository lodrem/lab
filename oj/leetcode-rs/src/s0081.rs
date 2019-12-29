pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mut start_idx = 0;
        let mut end_idx = nums.len();

        while start_idx < end_idx {
            let mid_idx = (end_idx - start_idx) / 2 + start_idx;

            let start = nums[start_idx];
            let middle = nums[mid_idx];
            let end = nums[end_idx - 1];

            if middle == target {
                return true;
            }

            if start == end && start == middle {
                start_idx += 1;
                end_idx -= 1;
            } else if start < end {
                if target < middle {
                    end_idx = mid_idx;
                } else {
                    start_idx = mid_idx + 1;
                }
            } else {
                if start <= middle {
                    if start <= target && target < middle {
                        end_idx = mid_idx;
                    } else {
                        start_idx = mid_idx + 1;
                    }
                } else {
                    if middle < target && target <= end {
                        start_idx = mid_idx + 1;
                    } else {
                        end_idx = mid_idx;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), true);

        assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), true);
        assert_eq!(Solution::search(vec![3, 1], 3), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(Solution::search(vec![1, 1, 1, 3, 1], 3), true);
    }
}
