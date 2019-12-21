pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // f[i][j] => word1[0..i] -> word2[0..j]
        // if word1[i] == word2[j]
        // f[i][j] = 1 + min(f[i-1][j], f[i][j-1], f[i-1][j-1] - 1)
        // else
        // f[i][j] = 1 + min(f[i-1][j], f[i][j-1], f[i-1][j-1])

        // f[i][0] = i
        // f[0][j] = j

        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();

        let mut f = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        for i in 0..=word1.len() {
            f[i][0] = i;
        }

        for j in 0..=word2.len() {
            f[0][j] = j;
        }

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                let x = f[i - 1][j] + 1; // word1 replace
                let y = f[i][j - 1] + 1; // word2 replace
                let z = f[i - 1][j - 1];

                f[i][j] = if word1[i - 1] == word2[j - 1] {
                    std::cmp::min(x, std::cmp::min(y, z))
                } else {
                    std::cmp::min(x, std::cmp::min(y, z + 1))
                };
            }
        }

        f[word1.len()][word2.len()] as i32
    }
}
