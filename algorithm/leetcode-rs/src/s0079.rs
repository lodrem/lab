pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || word.is_empty() {
            return false;
        }

        let word: Vec<char> = word.chars().collect();

        let rows = board.len();
        let cols = board[0].len();

        let mut path = vec![vec![false; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if Self::find(&board, i, j, &word, 0, &mut path) {
                    return true;
                }
            }
        }

        false
    }

    fn find(
        board: &[Vec<char>],
        x: usize,
        y: usize,
        word: &[char],
        word_idx: usize,
        path: &mut [Vec<bool>],
    ) -> bool {
        path[x][y] = true;
        if board[x][y] == word[word_idx] {
            if word_idx == word.len() - 1 {
                return true;
            } else {
                if x > 0 && !path[x - 1][y] {
                    if Self::find(board, x - 1, y, word, word_idx + 1, path) {
                        return true;
                    }
                }

                if y > 0 && !path[x][y - 1] {
                    if Self::find(board, x, y - 1, word, word_idx + 1, path) {
                        return true;
                    }
                }

                if x + 1 < board.len() && !path[x + 1][y] {
                    if Self::find(board, x + 1, y, word, word_idx + 1, path) {
                        return true;
                    }
                }

                if y + 1 < board[0].len() && !path[x][y + 1] {
                    if Self::find(board, x, y + 1, word, word_idx + 1, path) {
                        return true;
                    }
                }
            }
        }
        path[x][y] = false;
        false
    }
}
