pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();

        // f[i][j] indicates s[:i] match p[:j]
        // 1. s[i] == p[j] => f[i][j] = f[i-1][j-1]
        // 2. p[j] == '.' => f[i][j] = f[i-1][j-1]
        // 3. p[j] == '*' =>
        // 3.1. s[i] == p[j-1] or p[j-1] == '.' =>
        //          f[i][j] = f[i][j-2] (empty, count(*) == 0)
        //          f[i][j] = f[i][j-1] (single, count(*) == 1)
        //          f[i][j] = f[i-1][j] (multiple, count(*) > 1)
        // 3.1. s[i] != p[j-1] and p[j-1] !+ '.' => f[i][j] = f[i][j-2]
        let mut f = vec![vec![false; p.len() + 1]; s.len() + 1];
        f[0][0] = true;

        for j in 0..p.len() {
            if p[j] == b'*' && f[0][j - 1] {
                f[0][j + 1] = true;
            }
        }

        for i in 0..s.len() {
            for j in 0..p.len() {
                if s[i] == p[j] || p[j] == b'.' {
                    f[i + 1][j + 1] = f[i][j];
                } else if p[j] == b'*' {
                    // p[j] == '*' indicates j > 0
                    f[i + 1][j + 1] = if s[i] == p[j - 1] || p[j - 1] == b'.' {
                        f[i + 1][j - 1] || f[i + 1][j] || f[i][j + 1]
                    } else {
                        f[i + 1][j - 1]
                    }
                }
            }
        }

        f[s.len()][p.len()]
    }
}
