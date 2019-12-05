pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        };
        let m = matrix.len();
        let n = matrix[0].len();

        let mut left = vec![0; n]; // initialize left as the leftmost boundary possible
        let mut right = vec![n; n];
        let mut height = vec![0; n];

        let mut maxarea = 0;
        for i in 0..m {
            let mut cur_left = 0;
            let mut cur_right = n;
            // update height
            for j in 0..n {
                if matrix[i][j] == '1' {
                    height[j] += 1;
                } else {
                    height[j] = 0;
                }
            }
            // update left
            for j in 0..n {
                if matrix[i][j] == '1' {
                    left[j] = std::cmp::max(left[j], cur_left);
                } else {
                    left[j] = 0;
                    cur_left = j + 1;
                }
            }
            // update right
            for j in (0..n).rev() {
                if matrix[i][j] == '1' {
                    right[j] = std::cmp::min(right[j], cur_right);
                } else {
                    right[j] = n;
                    cur_right = j;
                }
            }
            // update area
            for j in 0..n {
                maxarea = std::cmp::max(maxarea, (right[j] - left[j]) * height[j]);
            }
        }
        return maxarea as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );

        assert_eq!(
            Solution::maximal_rectangle(vec![vec!['1'], vec!['1'], vec!['1'], vec!['1']]),
            4
        );
    }
}
