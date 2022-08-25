pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let height = matrix.len();
        let width = matrix[0].len();

        let mut max_area = 0;
        let mut heights = vec![0; width];
        let mut left = vec![0; width];
        let mut right = vec![width; width];

        for h in 0..height {
            for i in 0..width {
                if matrix[h][i] == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }

            let mut left_idx = 0;
            for i in 0..width {
                if matrix[h][i] == '1' {
                    left[i] = std::cmp::max(left[i], left_idx);
                } else {
                    left[i] = 0;
                    left_idx = i + 1;
                }
            }

            let mut right_idx = width;
            for i in (0..width).rev() {
                if matrix[h][i] == '1' {
                    right[i] = std::cmp::min(right[i], right_idx);
                } else {
                    right[i] = width;
                    right_idx = i;
                }
            }

            for i in 0..width {
                max_area = std::cmp::max(max_area, (right[i] - left[i]) * heights[i]);
            }
        }

        return max_area as i32;
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
