pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn backtrack(tmp: &mut Vec<i32>, candidates: &[i32], target: i32, res: &mut Vec<Vec<i32>>) {
        for i in 0..candidates.len() {
            if i > 0 && candidates[i] == candidates[i - 1] {
                continue;
            }

            let candidate = candidates[i];

            if candidate > target {
                break;
            }

            tmp.push(candidate);
            if candidate == target {
                res.push(tmp.clone());
            } else {
                Self::backtrack(tmp, &candidates[i + 1..], target - candidate, res);
            }
            tmp.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        candidates.sort();

        Self::backtrack(&mut vec![], &candidates, target, &mut res);

        res
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );

        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
