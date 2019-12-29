pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut start_idx = 0;
        let mut end_idx = nums.len();

        while start_idx < end_idx {
            let mid_idx = (end_idx - start_idx) / 2 + start_idx;

            let start = nums[start_idx];
            let middle = nums[mid_idx];
            let end = nums[end_idx - 1];

            if middle == target {
                return mid_idx as i32;
            }

            if start < end {
                if target < middle {
                    end_idx = mid_idx;
                } else {
                    start_idx = mid_idx + 1;
                }
            } else {
                if start < middle {
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

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);

        assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
        assert_eq!(Solution::search(vec![3, 1], 3), 0);
    }
}
