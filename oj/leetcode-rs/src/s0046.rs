pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn backtrack(first: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            result.push(nums.clone());
        }

        for i in first..nums.len() {
            nums.swap(first, i);
            Self::backtrack(first + 1, nums, result);

            nums.swap(first, i);
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // p(n) = n!
        let p = {
            let mut p = 1;
            for i in 2..nums.len() + 1 {
                p *= i;
            }
            p
        };

        let mut res = Vec::with_capacity(p);

        Self::backtrack(0, &mut nums, &mut res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2]
            ]
        );
    }
}
