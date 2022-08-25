pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn backtrack(tmp: &mut Vec<i32>, len: usize, candidates: &[i32], res: &mut Vec<Vec<i32>>) {
        for i in 0..candidates.len() {
            tmp.push(candidates[i]);

            if tmp.len() == len {
                res.push(tmp.clone());
            } else {
                let candidates = &candidates[i + 1..];
                Self::backtrack(tmp, len, candidates, res);
            }

            tmp.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let candidates: Vec<i32> = (1..=n).collect();

        Self::backtrack(
            &mut Vec::with_capacity(k as usize),
            k as usize,
            &candidates,
            &mut res,
        );

        res
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }
}
