pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let is_odd = (nums1.len() + nums2.len()) % 2 == 1;
        let median_idx = (nums1.len() + nums2.len()) / 2;

        let mut median: f64 = 0.0;

        let mut i = 0;
        let mut j = 0;

        loop {
            let cur = match (i >= nums1.len(), j >= nums2.len()) {
                (true, true) => break,
                (true, false) => {
                    j += 1;
                    nums2[j - 1]
                }
                (false, true) => {
                    i += 1;
                    nums1[i - 1]
                }
                _ => {
                    if nums1[i] < nums2[j] {
                        i += 1;
                        nums1[i - 1]
                    } else {
                        j += 1;
                        nums2[j - 1]
                    }
                }
            };

            let step = i + j - 1;

            if step == median_idx {
                median += cur as f64;
            } else if !is_odd && step == median_idx - 1 {
                median += cur as f64;
            } else if step > median_idx {
                break;
            }
        }

        if is_odd {
            median
        } else {
            median / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
