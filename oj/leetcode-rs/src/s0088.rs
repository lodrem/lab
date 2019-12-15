pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut inserted_idx = m + n - 1;

        let mut i = m - 1;
        let mut j = n - 1;

        while inserted_idx >= 0 {
            if i >= 0 && j >= 0 {
                nums1[inserted_idx as usize] = if nums1[i as usize] > nums2[j as usize] {
                    i -= 1;
                    nums1[(i + 1) as usize]
                } else {
                    j -= 1;
                    nums2[(j + 1) as usize]
                };
            } else if i >= 0 {
                nums1[inserted_idx as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[inserted_idx as usize] = nums2[j as usize];
                j -= 1;
            }
            inserted_idx -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        {
            let mut nums1 = vec![1, 2, 3, 0, 0, 0];
            let mut nums2 = vec![2, 5, 6];
            Solution::merge(&mut nums1, 3, &mut nums2, 3);
            assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
        }
    }
}
