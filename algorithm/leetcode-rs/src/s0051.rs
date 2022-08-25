pub struct Solution;

struct SolutionState {
    n: usize,
    pub solutions: Vec<Vec<String>>,

    queens: Vec<i32>,

    rows: Vec<bool>,
    hills: Vec<bool>,
    dales: Vec<bool>,
}

impl SolutionState {
    fn could_place(&self, row: usize, col: usize) -> bool {
        !(self.rows[col] || self.hills[row + col] || self.dales[self.n + row - col])
    }

    fn place_queen(&mut self, row: usize, col: usize) {
        self.queens[row] = col as i32;
        self.rows[col] = true;
        self.hills[row + col] = true;
        self.dales[self.n + row - col] = true;
    }

    fn remove_queen(&mut self, row: usize, col: usize) {
        self.queens[row] = -1;
        self.rows[col] = false;
        self.hills[row + col] = false;
        self.dales[self.n + row - col] = false;
    }

    fn add_solution(&mut self) {
        let mut solution = Vec::with_capacity(self.n);
        for col in &self.queens {
            let col = *col as usize;
            let mut s = String::with_capacity(self.n);
            for _ in 0..col {
                s.push('.');
            }
            s.push('Q');
            for _ in col + 1..self.n {
                s.push('.');
            }
            solution.push(s);
        }

        self.solutions.push(solution);
    }

    fn backtrack(&mut self, row: usize) {
        for col in 0..self.n {
            if self.could_place(row, col) {
                self.place_queen(row, col);

                if row == self.n - 1 {
                    self.add_solution();
                } else {
                    self.backtrack(row + 1);
                }

                self.remove_queen(row, col);
            }
        }
    }

    fn new(n: usize) -> Self {
        Self {
            n,
            solutions: vec![],

            queens: vec![-1; n],

            rows: vec![false; n],
            hills: vec![false; 2 * n - 1],
            dales: vec![false; 4 * n - 1],
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut state = SolutionState::new(n as usize);
        state.backtrack(0);
        state.solutions
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        {
            let expected = vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ];
            assert_eq!(expected, Solution::solve_n_queens(4));
        }
    }
}
