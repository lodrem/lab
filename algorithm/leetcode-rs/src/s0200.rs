pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        use std::collections::VecDeque;

        let mut num = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    num += 1;

                    let mut island_fields = VecDeque::new();
                    island_fields.push_back((i, j));

                    while let Some((x, y)) = island_fields.pop_front() {
                        if grid[x][y] == '1' {
                            if x < grid.len() - 1 {
                                island_fields.push_back((x + 1, y));
                            }
                            if x > 0 {
                                island_fields.push_back((x - 1, y));
                            }
                            if y < grid[x].len() - 1 {
                                island_fields.push_back((x, y + 1));
                            }
                            if y > 0 {
                                island_fields.push_back((x, y - 1));
                            }
                            grid[x][y] = '0';
                        }
                    }
                }
            }
        }

        num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ])
        );

        assert_eq!(
            3,
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ])
        );
    }
}
