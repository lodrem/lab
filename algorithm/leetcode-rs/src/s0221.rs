pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        // f[i, j] = max(f[i-1, j], f[i-1, j-1], f[i, j-1]) + 1

        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };

        let mut f = vec![vec![0; cols + 1]; rows + 1];

        let mut res = 0;

        for i in 1..=rows {
            for j in 1..=cols {
                if matrix[i - 1][j - 1] == '1' {
                    use std::cmp::{max, min};
                    f[i][j] = min(min(f[i - 1][j], f[i][j - 1]), f[i - 1][j - 1]) + 1;
                    res = max(res, f[i][j]);
                }
            }
        }

        res * res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::maximal_square(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_square(vec![]), 0);
        assert_eq!(
            Solution::maximal_square(vec![vec!['1', '0'], vec!['1', '0']]),
            1
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '1', '1', '1'],
                vec!['1', '1', '1', '1'],
                vec!['1', '1', '1', '1']
            ]),
            9
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0'],
                vec!['1', '0', '1', '1'],
                vec!['1', '0', '1', '1'],
                vec!['1', '1', '1', '1']
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
    }
}
