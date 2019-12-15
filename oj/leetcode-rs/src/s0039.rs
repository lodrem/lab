pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn backtrack(prev_candidate: i32, candidates: &Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for (_, candidate) in candidates.iter().enumerate() {
            if *candidate < prev_candidate {
                continue;
            }

            if *candidate == target {
                res.push(vec![*candidate]);
                break;
            }

            if *candidate < target {
                res.extend(
                    Self::backtrack(*candidate, &candidates, target - *candidate)
                        .into_iter()
                        .map(|mut r| {
                            r.insert(0, *candidate);
                            r
                        }),
                );
            }
        }

        res
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        Self::backtrack(0, &candidates, target)
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );

        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }
}
